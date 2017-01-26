use ::network::Network;
use state_proto::state::NetworkConfig;
use state_proto::state::State;
use state_proto::state::Time;
use state_proto::state::Time_TimeMode;
use service_proto::service::Resource;
use player_proto::player::NetworkPlayer;
use player_proto::player::PlayerData;
use protobuf;
use protobuf::Message;
use protobuf::RepeatedField;
use super::TransientState;
use time::PreciseTime;
use itertools::Itertools;
use std::collections::HashMap;

pub struct RunningGame {
  state: State,
  players: PlayerData,
  transient: TransientState,
  last_run_time: PreciseTime,
}

fn resource_list_into_map(mut resources: RepeatedField<Resource>) -> HashMap<String, Resource>{
  let mut result = HashMap::new();

  resources.into_iter()
    .filter(|resource| {
      let is_unnamed = resource.get_name().is_empty();
      if is_unnamed {
        warn!("Dropped some unnamed resource on hotload")
      }
      !is_unnamed
    })
    .foreach(|resource| {
      let name = resource.get_name().to_owned();
      if result.contains_key(&name) {
        warn!("Dropped the second instance of resource {}", name);
      } else {
        result.insert(name, resource);
      }
    });

  result
}

fn try_parse_player_resource(mut resource: Option<Resource>) -> PlayerData {
  if resource.is_none() {
    info!("Player resource data not present, building fresh");
    return PlayerData::new();
  }

  let res = resource.unwrap();

  match protobuf::parse_from_bytes(res.get_data()) {
    Ok(data) => data,
    _ => {
      warn!("Could not parse last PlayerData, building fresh");
      PlayerData::new()
    }
  }
}

impl RunningGame {
  pub fn new(mut state: State, transient: TransientState) -> RunningGame {
    trace!("Hotloading with: {:?}", state);
    let resources = state.take_resources();
    let mut resource_map = resource_list_into_map(resources);
    let player_resource = try_parse_player_resource(resource_map.remove("players"));
    RunningGame {
      state: state,
      players: PlayerData::new(),
      transient: transient,
      last_run_time: PreciseTime::now(),
    }
  }

  pub fn run(&mut self) {
    let now = PreciseTime::now();

    let delta = self.last_run_time.to(now.clone());

    self.last_run_time = now;
    let microsecond_delta = delta.num_microseconds()
      .expect("time between runs was way too long (over 280k years!)");
    let next_timestamp = self.state.get_time().get_timestamp().clone() + microsecond_delta;
    self.state.mut_time().set_timestamp(next_timestamp);
  }

  pub fn build_state(&self) -> State {
    let mut state = self.state.clone();
    let mut player_resource = Resource::new();
    player_resource.set_name("players".to_owned());
    player_resource.set_data(self.players.write_to_bytes().expect("Couldn't write players proto"));
    state.mut_resources().push(player_resource);
    state
  }

  pub fn take_transient_state(self) -> TransientState {
    self.transient
  }
}
