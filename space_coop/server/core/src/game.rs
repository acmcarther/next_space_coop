use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use state_proto::state::NetworkConfig;
use state_proto::state::State;

use clap::ArgMatches;
use protobuf;
use protobuf::Message;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::io;
use std::str::FromStr;

pub struct GameServer {
  state: State,
}

impl GameServer {
  pub fn new() -> GameServer {
    let mut server = GameServer {
      state: State::new()
    };

    server.try_load_snapshot();
    server
  }

  pub fn set_flags(&mut self, matches: ArgMatches) {
    let port = matches.value_of("port")
      .and_then(|v| u16::from_str(&v).ok()).unwrap();
    self.state.mut_network().set_port(port as i32);
  }

  pub fn initialize(&mut self, state: State) {
    self.state = state;
  }

  pub fn dump_state(&mut self) -> State {
    self.state.clone()
  }

  pub fn run(&mut self) {
    let next_timestamp = self.state.get_time().get_timestamp().clone() + 1;
    self.state.mut_time().set_timestamp(next_timestamp);
    trace!("ran with internal timestamp: {:?}", self.state.get_time().get_timestamp());

    self.try_save_snapshot();
  }

  fn try_load_snapshot(&mut self) {
    let mut snapshot_file = env::temp_dir();
    snapshot_file.push("space_coop-server.snapshot");

    let state: Option<State> = File::open(snapshot_file)
      .ok()
      .and_then(|mut f| protobuf::parse_from_reader(&mut f).ok());

    if let Some(s) = state {
      self.state = s;
      let t = self.state.get_time().get_timestamp();
      info!("loaded from snapshot with timestamp: {}", t);
    }
  }

  fn try_save_snapshot(&mut self) {
    if self.state.get_time().get_timestamp() % 100 != 0 {
      return
    }

    let mut snapshot_file = env::temp_dir();
    snapshot_file.push("space_coop-server.snapshot");


    match (File::create(snapshot_file.clone()).ok(), self.state.write_to_bytes().ok()) {
      (Some(mut file), Some(bytes)) => {
        file.write_all(&bytes);
        trace!("wrote snap to {:?}", snapshot_file);
      },
      _ => {
        trace!("failed to write snap to {:?}", snapshot_file);
      }
    }
  }
}
