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


pub use ffi::*;

use game::GameServer;
use service::Service;
use state_proto::state::NetworkConfig;
use state_proto::state::State;
use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use std::io;
use std::io::Write;
use std::mem;

use std::ptr;

use std::sync::Mutex;

lazy_static! {
  static ref GAME_STATE: Mutex<GameServer> = {
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
    Mutex::new(GameServer::new())
  };
}

mod ffi {
  use ::clap::ArgMatches;
  use game::OpaqueState;
  use ::libc;
  use ::protobuf;
  use ::protobuf::Message;
  use state_proto::state::State;
  use std::io;
  use std::io::Write;
  use std::mem;
  use std::str::FromStr;
  use super::GAME_STATE;

  #[no_mangle]
  pub fn set_flags(matches: ArgMatches) {
    GAME_STATE.lock().unwrap().set_flags(matches);
    info!("Set flags for dylib");
  }

  #[no_mangle]
  pub fn initialize(state: *mut libc::c_void) {
    let mut opaque_state = unsafe { Box::from_raw(state as *mut OpaqueState) };
    GAME_STATE.lock().unwrap().initialize(opaque_state);
  }

  #[no_mangle]
  pub fn run() {
    GAME_STATE.lock().unwrap().run();
  }

  #[no_mangle]
  pub fn dump_state() -> *mut libc::c_void {
    let opaque_state = GAME_STATE.lock().unwrap().dump_state();
    Box::into_raw(Box::new(opaque_state)) as *mut libc::c_void
  }

}
