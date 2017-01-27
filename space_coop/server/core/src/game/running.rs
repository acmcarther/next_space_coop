use clap::ArgMatches;
use ::network::Network;
use protocol_proto::protocol::Packet;
use protocol_proto::protocol::Packet_MessageType;
use network_proto::network::ClientMessage;
use network_proto::network::ClientMessage_MessageType;
use network_proto::network::ServerMessage;
use network_proto::network::ServerMessage_MessageType;
use service_proto::service::Resource;
use player_proto::player::PlayerData;
use protobuf;
use protobuf::Message;
use protobuf::RepeatedField;
use time::PreciseTime;
use itertools::Itertools;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use gaffer_udp::packet::GafferPacket;

/**
 * The exported type that is passed unmodified between loads
 *
 * WARNING: CHANGING THE SIGNATURE OF THIS TYPE MAY BREAK HOTLOADS.
 */
pub type Transient = Network;

/**
 * The exported type that is serialized and deserialized
 *
 * Changing the signature of this is safe, provided it has a flexible serialization format, such as
 * protobuf.
 */
pub use state_proto::state::State;

pub struct RunningGame {
  state: State,
  players: PlayerData,
  network: Network,
  last_run_time: PreciseTime,
}

fn keep_alive_packet() -> Vec<u8> {
  let mut packet = Packet::new();
  packet.set_message_type(Packet_MessageType::KEEP_ALIVE);
  packet.write_to_bytes().expect("couldn't convert packet to bytes")
}

fn connected_packet() -> Vec<u8> {
  let mut packet = Packet::new();
  packet.set_message_type(Packet_MessageType::DATAGRAM);
  let mut msg = ServerMessage::new();
  msg.set_message_type(ServerMessage_MessageType::CONNECTED);
  packet.set_payload(msg.write_to_bytes().expect("Couldnt convert message to bytes"));

  packet.write_to_bytes().expect("couldn't convert packet to bytes")
}

fn disconnected_packet() -> Vec<u8> {
  let mut packet = Packet::new();
  packet.set_message_type(Packet_MessageType::DATAGRAM);
  let mut msg = ServerMessage::new();
  msg.set_message_type(ServerMessage_MessageType::DISCONNECTED);
  packet.set_payload(msg.write_to_bytes().expect("Couldnt convert message to bytes"));

  packet.write_to_bytes().expect("couldn't convert packet to bytes")
}

fn resource_list_into_map(resources: RepeatedField<Resource>) -> HashMap<String, Resource>{
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

fn try_parse_player_resource(resource: Option<Resource>) -> PlayerData {
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
  pub fn fresh(flags: ArgMatches) -> RunningGame {
    RunningGame::from_snapshot(State::new(), flags)
  }

  pub fn from_snapshot(state: State, flags: ArgMatches) -> RunningGame {
    let port = flags
      .value_of("port")
      .and_then(|v| u16::from_str(&v).ok())
      .unwrap();
    RunningGame::new(state, Network::new(port))
  }

  pub fn from_opaque(state: State, network: Network) -> RunningGame {
    RunningGame::new(state, network)
  }

  fn new(mut state: State, network: Network) -> RunningGame {
    trace!("Hotloading with: {:?}", state);
    let resources = state.take_resources();
    let mut resource_map = resource_list_into_map(resources);
    let player_resource = try_parse_player_resource(resource_map.remove("players"));
    RunningGame {
      state: state,
      players: player_resource,
      network: network,
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

    self.update_network_state();
  }

  pub fn build_state(&self) -> State {
    let mut state = self.state.clone();
    let mut player_resource = Resource::new();
    player_resource.set_name("players".to_owned());
    player_resource.set_data(self.players.write_to_bytes().expect("Couldn't write players proto"));
    state.mut_resources().push(player_resource);
    state
  }

  pub fn build_transient(self) -> Transient {
    self.network
  }

  fn update_network_state(&mut self) {
    let mut ip_to_player_idx: HashMap<String, usize> = HashMap::new();
    self.players.get_players().iter()
      .enumerate()
      .foreach(|(idx, player)| {
        ip_to_player_idx.insert(player.get_ip().to_owned(), idx);
      });

    let mut outgoing_msgs = Vec::new();

    while let Ok(Some(pkt)) = self.network.socket.recv() {
      let addr_str = pkt.addr.to_string();
      let idx_opt = ip_to_player_idx.get(&addr_str).cloned();

      let idx = if idx_opt.is_none() {
        let map_size = ip_to_player_idx.len();
        ip_to_player_idx.insert(addr_str.clone(), map_size);
        map_size
      } else {
        idx_opt.unwrap().clone()
      };


      let player = self.players.mut_players().get_mut(idx).unwrap();

      let message_opt = protobuf::parse_from_bytes::<Packet>(&pkt.payload).ok();
      if message_opt.is_some() {
        let message = message_opt.unwrap();
        match message.get_message_type() {
          Packet_MessageType::DATAGRAM => {
            let payload_bytes = message.get_payload();
            let msg_opt = protobuf::parse_from_bytes::<ClientMessage>(payload_bytes).ok();
            if let Some(msg) = msg_opt {
              match msg.get_message_type() {
                ClientMessage_MessageType::CONNECT => {
                  player.set_connected(true);
                  outgoing_msgs.push(GafferPacket::new(SocketAddr::from_str(&addr_str).unwrap(), connected_packet()))
                },
                ClientMessage_MessageType::DISCONNECT => {
                  player.set_connected(false);
                  outgoing_msgs.push(GafferPacket::new(SocketAddr::from_str(&addr_str).unwrap(), disconnected_packet()))
                },
                _ => {}
              }
            }
          }
          _ => {}
        }

      }
      outgoing_msgs.push(GafferPacket::new(SocketAddr::from_str(&addr_str).unwrap(), keep_alive_packet()))
    }

    outgoing_msgs.into_iter().foreach(|msg| {
      if self.network.socket.send(msg).is_err() {
        warn!("Failed to enqueue network message")
      }
    });
  }
}
