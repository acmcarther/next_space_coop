extern crate state_proto;
extern crate service_proto;
extern crate player_proto;
extern crate protocol_proto;
extern crate network_proto;
extern crate service;
extern crate libc;
extern crate clap;
extern crate protobuf;
extern crate gaffer_udp;
extern crate fern;
extern crate time;
extern crate itertools;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod game;
mod network;

use game::SnapshottedGame;
use game::running::State;
use game::running::Transient;
use game::running::RunningGame;
use std::sync::Mutex;

lazy_static! {
  static ref GAME_STATE: Mutex<Option<SnapshottedGame<State, Transient, RunningGame>>> = {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            // This is a fairly simple format, though it's possible to do more complicated ones.
            // This closure can contain any code, as long as it produces a String message.
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("logs/server.log")],
        level: log::LogLevelFilter::Trace,
    };

    fern::init_global_logger(logger_config, log::LogLevelFilter::Trace)
      .expect("could not load logger");
    Mutex::new(None)
  };
}

pub use ffi::*;
mod ffi {
  use clap::ArgMatches;
  use game::SnapshottedGame;
  use libc;
  use super::GAME_STATE;

  static APP_NAME: &'static str = "spacecoop-server";

  #[no_mangle]
  pub fn new(matches: ArgMatches) {
    let mut state = GAME_STATE.lock().unwrap();
    *state = Some(SnapshottedGame::new(APP_NAME, matches));
  }

  #[no_mangle]
  pub fn hotload(opaque_state: *mut libc::c_void) {
    let mut state = GAME_STATE.lock().unwrap();
    *state = Some(SnapshottedGame::hotload(APP_NAME, opaque_state));
  }

  #[no_mangle]
  pub fn run() {
    let mut state = GAME_STATE.lock().unwrap();
    state.as_mut()
      .expect("must call new or hotload before run")
      .run()
  }

  #[no_mangle]
  pub fn dump_state() -> *mut libc::c_void {
    let mut state = GAME_STATE.lock().unwrap();
    state.take()
      .expect("must call new or hotload before dump_state")
      .dump_state()
  }

}
