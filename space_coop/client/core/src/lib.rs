extern crate dylib_core;
extern crate client_state;

use client_state::state::State;

struct TotalState;
struct OpaqueState;

struct Client {
  pub state: State
}

impl Client {
}

impl Game<State, ()> for Client {
  fn new(_: ArgMatches) -> Client {
    Client::new(State::new())
  }

  fn new_from_hotload(persistent: State, _: ()) {
    Client::new(State::new())
  }

  fn copy_persistent_state(&self) -> State {
    client.state.clone()
  }

  fn take_transient_state(self) -> () {
    ()
  }
}

lazy_static! {
  static ref STATE: Mutex<DylibFFI<State, (), Client>>> = {
    let logger_config = dylib_core::get_default_logger_config("./log/client.log")

    fern::init_global_logger(logger_config, log::LogLevelFilter::Trace)
      .expect("could not load logger");

    Mutex::new(DylibGame::new());
  }
}

generate_ffi!(STATE)
