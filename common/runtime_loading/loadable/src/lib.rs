extern crate libc;
extern crate protobuf;
extern crate clap;

#[macro_use]
extern crate log;

use clap::ArgMatches;
use std::env;
use std::path::PathBuf;
use std::marker::PhantomData;
use std::fs::File;
use std::io::Write;

#[macro_export]
macro_rules! generate_ffi {
  () => {
    mod ffi {
      use ::clap::ArgMatches;
      use $crate::SnapshottedGame;
      use ::libc;

      // Must be Mutex<Option<SnapshottedGame<..>>
      use super::GAME_STATE;

      // Must be &str
      use super::APP_NAME;

      #[no_mangle]
      pub fn new(matches: ArgMatches) {
        let mut state = GAME_STATE.lock().unwrap();
        *state = Some(SnapshottedGame::new(APP_NAME, matches));
        info!("Initialized dylib");
      }

      #[no_mangle]
      pub fn hotload(opaque_state: *mut libc::c_void) {
        let mut state = GAME_STATE.lock().unwrap();
        *state = Some(SnapshottedGame::hotload(APP_NAME, opaque_state));
        info!("Reloaded dylib");
      }

      #[no_mangle]
      pub fn run() {
        trace!("Running main function");
        let mut state = GAME_STATE.lock().unwrap();
        state.as_mut()
          .expect("must call new or hotload before run")
          .run()
      }

      #[no_mangle]
      pub fn dump_state() -> *mut libc::c_void {
        info!("Unloading dylib");
        let mut state = GAME_STATE.lock().unwrap();
        state.take()
          .expect("must call new or hotload before dump_state")
          .dump_state()
      }
    }
  }
}

/**
 * Wire state that is compatible with past or future versions of the same struct, for different
 * definitions of State.
 *
 * WARNING: CHANGING THE SIGNATURE OF THIS TYPE MAY BREAK HOTLOADS.
 */
pub struct OpaqueState<T> {
  state_bytes: Vec<u8>,
  pub transient: T,
}

impl<T> OpaqueState<T> {
  fn new<M: ::protobuf::MessageStatic>(state: M, transient: T) -> OpaqueState<T> {
    OpaqueState {
      state_bytes: state.write_to_bytes().expect("proto failed to serialize"),
      transient: transient
    }
  }
  fn parse_state_bytes<M: ::protobuf::MessageStatic>(&self) -> M {
    protobuf::parse_from_bytes(&self.state_bytes)
        .expect("last state proto was not parseable")
  }
}


fn new_snapshotter<S: ::protobuf::MessageStatic>(name: &str) -> Snapshotter<S> {
  let mut snap_path = env::temp_dir();
  snap_path.push(format!("{}.snapshot", name));
  Snapshotter::new(1000 /* rate */, snap_path)
}

pub struct SnapshottedGame<S: ::protobuf::MessageStatic, T, G: Game<S, T>> {
  game: G,
  snapshotter: Snapshotter<S>,
  _t: PhantomData<T>,
}

pub trait Game<S, T> {
  fn fresh(flags: ArgMatches) -> Self;
  fn from_snapshot(state: S, flags: ArgMatches) -> Self;
  fn from_opaque(state: S, transient: T) -> Self;
  fn run(&mut self);
  fn build_state(&self) -> S;
  fn build_transient(self) -> T;
}

impl<S: ::protobuf::MessageStatic, T, G: Game<S, T>> SnapshottedGame<S, T, G> {
  pub fn new(name: &str, flags: ArgMatches) -> SnapshottedGame<S, T, G> {
    let snapshotter = new_snapshotter(name);
    let game = if let Some(snapshot) = snapshotter.load() {
      G::from_snapshot(snapshot, flags)
    } else {
      G::fresh(flags)
    };

    SnapshottedGame {
      game: game,
      snapshotter: snapshotter,
      _t: PhantomData
    }
  }

  pub fn hotload(name: &str, opaque_state: *mut libc::c_void) -> SnapshottedGame<S, T, G> {
    let opaque_state = unsafe { Box::from_raw(opaque_state as *mut OpaqueState<T>) };
    SnapshottedGame {
      game: G::from_opaque(
        opaque_state.parse_state_bytes(),
        opaque_state.transient),
      snapshotter: new_snapshotter(name),
      _t: PhantomData
    }
  }

  pub fn dump_state(self) -> *mut libc::c_void {
    Box::into_raw(Box::new(OpaqueState::new(
      self.game.build_state(),
      self.game.build_transient()))) as *mut libc::c_void
  }

  pub fn run(&mut self) {
    self.game.run();
    let borrowed_game = &self.game;
    self.snapshotter.snap(|| borrowed_game.build_state())
  }
}

struct Snapshotter<S: ::protobuf::MessageStatic> {
  rate: u32,
  path: PathBuf,
  call_counter: u32,
  _msg: PhantomData<S>
}

impl<S: ::protobuf::MessageStatic> Snapshotter<S> {
  pub fn new(rate: u32, path: PathBuf) -> Snapshotter<S> {
    Snapshotter {
      rate: rate,
      path: path,
      call_counter: 0,
      _msg: PhantomData
    }
  }

  pub fn snap<F: Fn() -> S>(&mut self, state_generator: F) {
    if self.call_counter % self.rate == 0 {
      match (File::create(&self.path).ok(),
             state_generator().write_to_bytes().ok()) {
        (Some(mut file), Some(bytes)) => {
          if file.write_all(&bytes).is_ok() {
            trace!("Wrote snap to {:?}", &self.path);
          } else {
            warn!("Failed to write snap to {:?}", &self.path);
          }
        },
        _ => {
          trace!("Failed to write snap to {:?}", &self.path);
        },
      }
    }

    self.call_counter = self.call_counter.wrapping_add(1);
  }

  pub fn load(&self) -> Option<S> {
    let state: Option<S> = File::open(&self.path)
      .ok()
      .and_then(|mut f| protobuf::parse_from_reader(&mut f).ok());

    if state.is_some() {
      info!("Loaded from snapshot");
    } else {
      info!("Failed to load from snapshot");
    }

    state
  }
}
