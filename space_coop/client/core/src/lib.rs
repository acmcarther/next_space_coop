// TODO(acmcarther): this is copy pasta from space_coop/server
extern crate state_proto;
extern crate ash;
extern crate lite;
extern crate network_proto;
extern crate game_proto;
extern crate libc;
extern crate clap;
extern crate protobuf;
extern crate gaffer_udp;
extern crate fern;
extern crate time;
extern crate rand;
extern crate itertools;
#[macro_use]
extern crate loadable;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod game;

use loadable::SnapshottedGame;
use game::State;
use game::Transient;
use game::RunningGame;
use std::sync::Mutex;

fn init_logger() {
  let logger_config = fern::DispatchConfig {
      format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
          format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
      }),
      output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("logs/client.log")],
      level: log::LogLevelFilter::Trace,
  };

  fern::init_global_logger(logger_config, log::LogLevelFilter::Trace)
    .expect("could not load logger");

  info!("Started dylib logger");
}

lazy_static! {
  static ref GAME_STATE: Mutex<Option<SnapshottedGame<State, Transient, RunningGame>>> = {
    init_logger();
    Mutex::new(None)
  };
}

// Used by ffi macro below for naming output snapshot files
static APP_NAME: &'static str = "spacecoop-client";

generate_ffi! {}
pub use ffi::*;
