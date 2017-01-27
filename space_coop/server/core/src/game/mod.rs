mod running;


use clap::ArgMatches;
use network::Network;
use protobuf;
use protobuf::Message;
use self::running::RunningGame;
use state_proto::state::NetworkConfig;
use state_proto::state::State;
use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::marker::PhantomData;
use std::fs::File;
use std::io;
use std::io::Write;
use std::str::FromStr;
use time::PreciseTime;

/**
 * The "opaque pointer" that this dylib's state is cast to.
 *
 * WARNING: CHANGING THIS CLASS WHILE HOTLOADING CAN LEAD TO UNDEFINED BEHAVIOR
 */
pub struct OpaqueState {
  pub state_bytes: Vec<u8>,
  pub transient_state: TransientState,
}

/**
 * Per-execution properties that persist while hotloading, but can be dropped between snapshot
 * loads.
 *
 * WARNING: CHANGING THIS CLASS WHILE HOTLOADING CAN LEAD TO UNDEFINED BEHAVIOR
 */
pub struct TransientState {
  pub network: Network,
}

impl TransientState {
  pub fn new(network: Network) -> TransientState {
    TransientState { network: network }
  }
}

/**
 * The whole state of the game system.
 *
 * This struct is safe to change while hotloading
 */
pub struct GameServer {
  state: Option<State>,
  transient: Option<TransientState>,
  running_game: Option<RunningGame>,
  ticks: u64,
  snapshotter: Snapshotter<State>,
}

impl GameServer {
  pub fn new() -> GameServer {
    let mut snap_path = env::temp_dir();
    snap_path.push("space_coop-server.snapshot");

    let mut server = GameServer {
      state: None,
      transient: None,
      running_game: None,
      ticks: 0,
      snapshotter: Snapshotter::new(1000 /* rate */, snap_path),
    };
    server
  }

  /**
   * Updates the internal state to have the flags passed.
   *
   * Will crate a new initial state if one does not exist.
   * This function has no effect if the game is already running.
   */
  pub fn set_flags(&mut self, matches: ArgMatches) {
    let port = matches.value_of("port")
      .and_then(|v| u16::from_str(&v).ok())
      .unwrap();

    if let Some(state) = self.state.as_mut() {
      state.mut_network().set_port(port as i32);
      return;
    }

    let mut new_state = self.snapshotter.load()
      .unwrap_or(State::new());
    new_state.mut_network().set_port(port as i32);
    self.state = Some(new_state);
  }

  /**
   * Parses the persistent and transient state from the provided opaque state.
   *
   * This function has no effect if the game is already running.
   */
  pub fn initialize(&mut self, opaque_state: Box<OpaqueState>) {
    info!("Hotloading...");
    self.state = Some(protobuf::parse_from_bytes(&opaque_state.state_bytes).unwrap());
    self.transient = Some(opaque_state.transient_state);
  }

  /**
   * Unloads a running game, extracting the persistent and transient state.
   */
  pub fn dump_state(&mut self) -> OpaqueState {
    info!("Unloading running game");
    let running_game =
      self.running_game.take().expect("Tried to dump state from a non-running game");
    OpaqueState {
      state_bytes: running_game.build_state().write_to_bytes().unwrap(),
      transient_state: running_game.take_transient_state(),
    }
  }

  /**
   * Runs an existing RunningGame, if one already exists, or generates a fresh RunningGame, and
   * runs that.
   */
  pub fn run(&mut self) {
    self.ticks = self.ticks + 1;

    if self.running_game.is_some() {
      self.running_game.as_mut().unwrap().run();
    } else {
      let state = self.state
        .take()
        .or_else(|| self.snapshotter.load())
        .unwrap_or(State::new());

      let transient = self.transient
        .take()
        .unwrap_or_else(|| GameServer::start_transient(&state));
      let mut running_game = RunningGame::new(state, transient);
      info!("Started game");
      running_game.run();
      self.running_game = Some(running_game);
    }
    let game = &self.running_game;
    self.snapshotter.snap(|| {game.as_ref().unwrap().build_state()})
  }

  pub fn start_transient(state: &State) -> TransientState {
    info!("Initializing transient state objects");
    let network = Network::new(state.get_network().get_port() as u16);
    TransientState::new(network)
  }
}

pub struct Snapshotter<S: ::protobuf::MessageStatic> {
  rate: u32,
  path: PathBuf,
  call_counter: u32,
  _msg: PhantomData<S>
}

impl<S: ::protobuf::MessageStatic> Snapshotter<S> {
  fn new(rate: u32, path: PathBuf) -> Snapshotter<S> {
    Snapshotter {
      rate: rate,
      path: path,
      call_counter: 0,
      _msg: PhantomData
    }
  }

  fn snap<F: Fn() -> S>(&mut self, state_generator: F) {
    if self.call_counter % self.rate == 0 {
      match (File::create(&self.path).ok(),
             state_generator().write_to_bytes().ok()) {
        (Some(mut file), Some(bytes)) => {
          file.write_all(&bytes);
          trace!("Wrote snap to {:?}", &self.path);
        },
        _ => {
          trace!("Failed to write snap to {:?}", &self.path);
        },
      }
    }

    self.call_counter = self.call_counter.wrapping_add(1);
  }

  fn load(&self) -> Option<S> {
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
