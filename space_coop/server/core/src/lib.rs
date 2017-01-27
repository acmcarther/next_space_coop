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
extern crate runtime_loaded_game;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod game;
mod network;

use runtime_loaded_game::SnapshottedGame;
use game::State;
use game::Transient;
use game::RunningGame;
use std::sync::Mutex;

fn init_logger() {
  let logger_config = fern::DispatchConfig {
      format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
          format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
      }),
      output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("logs/server.log")],
      level: log::LogLevelFilter::Trace,
  };

  fern::init_global_logger(logger_config, log::LogLevelFilter::Info)
    .expect("could not load logger");

  info!("Started dylib logger");
}

lazy_static! {
  static ref GAME_STATE: Mutex<Option<SnapshottedGame<State, Transient, RunningGame>>> = {
    init_logger();
    Mutex::new(None)
  };
}

static APP_NAME: &'static str = "spacecoop-server";

generate_ffi! {}
pub use ffi::*;
