extern crate state_proto;
extern crate service;
extern crate libc;
extern crate clap;

#[macro_use]
extern crate lazy_static;

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

lazy_static! {
  static ref GAME_STATE: Mutex<State> = {
    Mutex::new(State::new())
  };
}

mod ffi {
  use ::libc;
  use super::GAME_STATE;
  use state_proto::state::State;
  use std::io;
  use std::io::Write;
  use std::mem;
  use std::str::FromStr;
  use ::clap::ArgMatches;

  #[no_mangle]
  pub fn set_flags(matches: ArgMatches) {
    let port = matches.value_of("port")
      .and_then(|v| u16::from_str(&v).ok()).unwrap();
    let mut state = GAME_STATE.lock().unwrap();
    state.mut_network().set_port(port as i32);
  }

  #[no_mangle]
  pub fn initialize(state: *mut libc::c_void) {
    use std::ops::DerefMut;

    let mut state = unsafe { Box::from_raw(state as *mut State) };
    mem::swap(GAME_STATE.lock().unwrap().deref_mut(), state.deref_mut());
  }

  #[no_mangle]
  pub fn run() {
    let mut state = GAME_STATE.lock().unwrap();

    let next_timestamp = state.get_time().get_timestamp().clone() + 1;

    state.mut_time().set_timestamp(next_timestamp);
    io::stdout().write(format!("timestamp: {:?}\n", state.get_time().get_timestamp()).as_bytes()).unwrap();
    io::stdout().flush().unwrap();
  }

  #[no_mangle]
  pub fn dump_state() -> *mut libc::c_void {
    Box::into_raw(Box::new(GAME_STATE.lock().unwrap().clone())) as *mut libc::c_void
  }

}
