extern crate state_proto;
extern crate service;
extern crate libc;
extern crate clap;
extern crate protobuf;

#[macro_use]
extern crate lazy_static;

mod game;

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
    io::stdout().write(b"hotload!\n").unwrap();
    io::stdout().flush().unwrap();
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
  use ::clap::ArgMatches;

  #[no_mangle]
  pub fn set_flags(matches: ArgMatches) {
    GAME_STATE.lock().unwrap().set_flags(matches);
  }

  #[no_mangle]
  pub fn initialize(state: *mut libc::c_void) {
    use std::ops::DerefMut;

    let mut state_bytes = unsafe { Box::from_raw(state as *mut Vec<u8>) };
    let state = protobuf::parse_from_bytes(&state_bytes).unwrap();
    GAME_STATE.lock().unwrap().initialize(state);
  }

  #[no_mangle]
  pub fn run() {
    GAME_STATE.lock().unwrap().run();
  }

  #[no_mangle]
  pub fn dump_state() -> *mut libc::c_void {
    let state = GAME_STATE.lock().unwrap().dump_state();
    let bytes = state.write_to_bytes().unwrap();

    Box::into_raw(Box::new(bytes)) as *mut libc::c_void
  }

}
