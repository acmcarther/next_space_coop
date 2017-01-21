extern crate state_proto;
extern crate service;
extern crate service_registry;
extern crate libc;

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

lazy_static! {
  static ref GAME_STATE: Mutex<State> = {
    Mutex::new(State::new())
  };
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

pub struct Core {
  // The internal state of the game core
  // This includes fundamental state, such as the current timestamp
  state: State
}

impl Core {
  // Load an all new core from nothing
  pub fn new() -> Core {
    // TODO(acmcarther): When I can parse text protos, represent the default state as a text proto
    // that we load in
    let mut init_state = State::new();
    let mut init_time = Time::new();
    let mut init_network = NetworkConfig::new();

    init_time.set_mode(Time_TimeMode::NORMAL);
    init_time.set_timestamp(0 as u64);
    init_network.set_port(8839 /* totally made up */);
    init_state.set_time(init_time);
    init_state.set_network(init_network);

    Core {
      state: init_state
    }
  }

  // Load a core from a prior state proto
  pub fn from_state(prior_state: State) -> Core {
    Core {
      state: prior_state
    }
  }

  // Load a core with just a port
  pub fn new_with_port(port: u16) -> Core {
    let mut default_core = Core::new();
    default_core.state.mut_network().set_port(port as i32);
    default_core
  }

  // Extract persistent state into a proto
  // NOTE: This *could* be used to introspect into core, but it should *not* be
  pub fn into_state(&self) -> State {
    self.state.clone()
  }

  // Run the game loop, indefinitely
  pub fn run(self) {
    // TODO(acmcarther): Do exciting things
  }
}
