use clap::ArgMatches;
use itertools::Itertools;
use loadable::Game;
use lite::LiteClient;
use game_proto::game::PlayerLike;
use game_proto::game::Snapshot;
use network_proto::network::ClientMessage;
use game_proto::game::GameMode;
use protobuf;
use protobuf::RepeatedField;
use network_proto::network::ServerMessage;
use network_proto::network::ServerMessage_MessageType;
use std::thread;
use time::Duration;
use time::SteadyTime;
use std::str::FromStr;

/**
 * The exported type that is passed unmodified between loads
 *
 * WARNING: CHANGING THE SIGNATURE OF THIS TYPE MAY BREAK HOTLOADS.
 */
pub struct Transient {
  pub network: LiteClient,
}


/**
 * The exported type that is serialized and deserialized
 *
 * Changing the definition of this is safe, since it is serialized and deserialized
 */
pub use state_proto::state::State;

pub const SIM_TARGET_TICKRATE: u32 = 100;
pub const NET_TARGET_TICKRATE: u32 = 1000;

lazy_static! {
  static ref SIM_INTERVAL_US: Duration =
    Duration::nanoseconds((1000000000.0 * (1.0 / SIM_TARGET_TICKRATE as f64)) as i64);
  static ref NET_INTERVAL_US: Duration =
    Duration::nanoseconds((1000000000.0 * (1.0 / NET_TARGET_TICKRATE as f64)) as i64);
}

pub struct RunningGame {
  state: State,
  network: LiteClient,
  last_sim_time: SteadyTime,
  last_net_time: SteadyTime,
  next_sim_time: SteadyTime,
  next_net_time: SteadyTime,
}

impl RunningGame {
  fn new(state: State, network: LiteClient) -> RunningGame {
    trace!("Hotloading with: {:?}", state);
    RunningGame {
      state: state,
      network: network,
      last_sim_time: SteadyTime::now(),
      last_net_time: SteadyTime::now(),
      next_sim_time: SteadyTime::now(),
      next_net_time: SteadyTime::now(),
    }
  }

  fn update_network_state(&mut self) {
    // TODO(acmcarther): stub
  }

  fn bridge_network_state_and_game_state(&mut self) {
    // TODO(acmcarther): stub
  }

  fn update_game_state(&mut self, microsecond_delta: i64) {
    // TODO(acmcarther): stub
  }

  fn bridge_game_state_and_network_state(&mut self) {
    // TODO(acmcarther): stub
  }
}


impl Game<State, Transient> for RunningGame {
  fn fresh(flags: ArgMatches) -> RunningGame {
    RunningGame::from_snapshot(State::new(), flags)
  }

  fn from_snapshot(state: State, flags: ArgMatches) -> RunningGame {
    let server_address = flags
      .value_of("server address")
      .unwrap();
    let client_address = flags
      .value_of("client address")
      .unwrap();
    RunningGame::new(state, LiteClient::new(server_address, client_address))
  }

  fn from_opaque(state: State, transient: Transient) -> RunningGame {
    RunningGame::new(state, transient.network)
  }

  fn run(&mut self) {
    let now = SteadyTime::now();

    if now > self.next_net_time {
      self.next_net_time = now + *NET_INTERVAL_US;
      self.last_net_time = now;
      self.update_network_state();
    }

    if now > self.next_sim_time {
      let delta = now.clone() - self.last_sim_time;
      self.next_sim_time = now + *SIM_INTERVAL_US;
      self.last_sim_time = now;

      let microsecond_delta = delta.num_microseconds()
        .expect("time between runs was way too long (over 280k years!)");

      let next_timestamp = self.state.get_timestamp().clone() + microsecond_delta;
      self.state.set_timestamp(next_timestamp);

      self.bridge_network_state_and_game_state();
      self.update_game_state(microsecond_delta);
      self.bridge_game_state_and_network_state();
    }
    thread::sleep(::std::time::Duration::new(0, 500000));
  }

  fn build_state(&self) -> State {
    self.state.clone()
  }

  fn build_transient(self) -> Transient {
    Transient {
      network: self.network
    }
  }
}
