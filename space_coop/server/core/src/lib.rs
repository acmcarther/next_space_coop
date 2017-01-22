extern crate state_proto;
extern crate service;
extern crate libc;
extern crate clap;
extern crate protobuf;
extern crate gaffer_udp;
extern crate fern;
extern crate time;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod game;
mod network;

use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use state_proto::state::NetworkConfig;
use state_proto::state::State;
use service::Service;

use std::ptr;
use std::io;
use std::io::Write;

use std::sync::Mutex;
use std::mem;

pub use ffi::*;

use game::GameServer;

lazy_static! {
  static ref GAME_STATE: Mutex<GameServer> = {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            // This is a fairly simple format, though it's possible to do more complicated ones.
            // This closure can contain any code, as long as it produces a String message.
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("logs/server.log")],
        level: log::LogLevelFilter::Info,
    };

    fern::init_global_logger(logger_config, log::LogLevelFilter::Trace)
      .expect("could not load logger");
    Mutex::new(GameServer::new())
  };
}

mod ffi {
  use ::libc;
  use ::protobuf;
  use ::protobuf::Message;
  use super::GAME_STATE;
  use state_proto::state::State;
  use std::io;
  use std::io::Write;
  use std::mem;
  use std::str::FromStr;
  use game::OpaqueState;
  use ::clap::ArgMatches;

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
