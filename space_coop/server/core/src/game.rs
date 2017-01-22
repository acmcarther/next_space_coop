use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use state_proto::state::NetworkConfig;
use state_proto::state::State;

use clap::ArgMatches;
use protobuf::Message;
use time::PreciseTime;
use protobuf;
use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::io;
use std::str::FromStr;
use network::Network;

/**
 * The "opaque pointer" that this dylib's state is cast to.
 *
 * WARNING: CHANGING THIS CLASS WHILE HOTLOADING CAN LEAD TO UNDEFINED BEHAVIOR
 */
pub struct OpaqueState {
  pub state_bytes: Vec<u8>,
  pub transient_state: TransientState,
}

/**
 * Per-execution properties that persist while hotloading, but can be dropped between snapshot
 * loads.
 *
 * WARNING: CHANGING THIS CLASS WHILE HOTLOADING CAN LEAD TO UNDEFINED BEHAVIOR
 */
pub struct TransientState {
  network: Network
}

impl TransientState {
  pub fn new(network: Network) -> TransientState {
    TransientState {
      network: network
    }
  }
}

/**
 * The whole state of the game system.
 *
 * This struct is safe to change while hotloading
 */
pub struct GameServer {
  state: State,
  last_run_time: PreciseTime,
  transient: Option<TransientState>,
}

impl GameServer {
  pub fn new() -> GameServer {
    let mut server = GameServer {
      state: State::new(),
      last_run_time: PreciseTime::now(),
      transient: None
    };
    server
  }

  pub fn set_flags(&mut self, matches: ArgMatches) {
    let port = matches.value_of("port")
      .and_then(|v| u16::from_str(&v).ok()).unwrap();
    self.state.mut_network().set_port(port as i32);
  }

  pub fn initialize(&mut self, opaque_state: Box<OpaqueState>) {
    self.state = protobuf::parse_from_bytes(&opaque_state.state_bytes).unwrap();
    self.transient = Some(opaque_state.transient_state);
    info!("Initialized dylib from prior state with timestamp {}", self.state.get_time().get_timestamp());
  }

  pub fn dump_state(&mut self) -> OpaqueState {
    info!("Unloading state for snapshot {}", self.state.get_time().get_timestamp());
    OpaqueState {
      state_bytes: self.state.write_to_bytes().unwrap(),
      transient_state: self.transient.take()
        .expect("tried to dump transient before installing it from a prior state, or initializing it"),
    }
  }

  pub fn run(&mut self) {
    self.try_to_load_first_time();

    let now = PreciseTime::now();
    let delta = self.last_run_time.to(now.clone());
    self.last_run_time = now;
    let microsecond_delta = delta.num_microseconds().expect("time between runs was way too long (over 280k years!)");

    let next_timestamp = self.state.get_time().get_timestamp().clone() + microsecond_delta;
    self.state.mut_time().set_timestamp(next_timestamp);
    trace!("Ran with internal timestamp: {:?}", self.state.get_time().get_timestamp());

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
      info!("Loaded from snapshot with timestamp: {}", t);
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
        trace!("Wrote snap to {:?}", snapshot_file);
      },
      _ => {
        trace!("Failed to write snap to {:?}", snapshot_file);
      }
    }
  }

  pub fn try_to_load_first_time(&mut self) {
    if self.transient.is_none() {
      info!("Initializing transient state objects");
      let network = Network::new(self.state.get_network().get_port() as u16);
      self.transient = Some(TransientState::new(network));
      info!("Attempting to load from snapshot");
      self.try_load_snapshot();
    }
  }
}
