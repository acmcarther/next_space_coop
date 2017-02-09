use clap::ArgMatches;
use network_proto::network::ClientMessage;
use network_proto::network::ClientMessage_MessageType;
use network_proto::network::ServerMessage;
use network_proto::network::ServerMessage_MessageType;
use state_proto::state::NetworkPlayer;
use protobuf;
use loadable::Game;
use protobuf::Message;
use protobuf::RepeatedField;
use time::PreciseTime;
use itertools::Itertools;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use gaffer_udp::packet::GafferPacket;
use lite::LiteServer;
use lite::LiteServerEvent;
use lite;

/**
 * The exported type that is passed unmodified between loads
 *
 * WARNING: CHANGING THE SIGNATURE OF THIS TYPE MAY BREAK HOTLOADS.
 */
pub type Transient = LiteServer;

/**
 * The exported type that is serialized and deserialized
 *
 * Changing the definition of this is safe, since it is serialized and deserialized
 */
pub use state_proto::state::State;

pub struct RunningGame {
  state: State,
  network: LiteServer,
  last_run_time: PreciseTime,
}

impl RunningGame {
  fn new(mut state: State, network: LiteServer) -> RunningGame {
    trace!("Hotloading with: {:?}", state);
    RunningGame {
      state: state,
      network: network,
      last_run_time: PreciseTime::now(),
    }
  }

  fn update_network_state(&mut self) {
    let mut outgoing_msgs = Vec::new();
    let mut incoming_msgs = self.network.read();

    while !incoming_msgs.is_empty() {
      outgoing_msgs.extend(self.process_messages(incoming_msgs));
      incoming_msgs = self.network.read();
    }

    outgoing_msgs.into_iter().foreach(|(client_id, msg)| {
      match self.network.write(client_id, msg) {
        lite::WriteResult::Success => {},
        lite::WriteResult::Failure_NotConnected => {
          trace!("Oops, tried to write a message to {}, who is disconnected.", client_id);
          self.find_or_establish_player(client_id).set_connected(false);
        },
      }
    });
  }

  fn process_messages(&mut self, events: Vec<LiteServerEvent>) -> Vec<(u32, Vec<u8>)> {
    use lite::LiteServerEvent::*;

    for event in events.into_iter() {
      match event {
        ConnectionEstablished { client_id } => {
          let player = self.find_or_establish_player(client_id);
          player.set_connected(true);
        }
        ConnectionDropped_Disconnect { client_id } | ConnectionDropped_Lag { client_id } => {
          let player = self.find_or_establish_player(client_id);
          player.set_connected(false);
        }
        Data { client_id, bytes } => {
          let player = self.find_or_establish_player(client_id);
          if player.get_connected() == false {
            trace!("Disconnected player {} is still talking. Is it a ghost?", client_id);
            continue;
          }

          let message_res = protobuf::parse_from_bytes::<ClientMessage>(&bytes);
          if message_res.is_err() {
            trace!("Player {} sent a malformed payload", client_id);
            continue;
          }

          let message = message_res.unwrap();
          match message.get_message_type() {
            ClientMessage_MessageType::UNKNOWN => {
              trace!("Player {} sent well formed unknown type payload.", client_id);
            },
            ClientMessage_MessageType::MOVE => {
              let move_data = message.get_move_data();
              trace!("Player {} tried to move to {} but we dont support moving yet.", client_id, move_data.get_y());
            }
          }
        }
      }
    }

    // We don't have anything to say yet.
    Vec::new()
  }

  fn find_or_establish_player(&mut self, client_id: u32) -> &mut NetworkPlayer {
    // TODO(acmcarther): Non-lexical lifetimes. This uses an intermediate idx because of lifetime
    // issues
    // TODO(acmcarther): Use a hash map if it even matters
    let player_idx = self.state.mut_players()
      .iter_mut()
      .enumerate()
      .filter(|&(_, ref p)| p.get_id() == client_id)
      .map(|(idx, _)| idx)
      .next()
      .unwrap_or_else(|| {
        let players = self.state.mut_players();
        let mut new_player = NetworkPlayer::new();
        new_player.set_id(client_id);
        players.push(new_player);
        players.len()
      });

    self.state.mut_players().get_mut(player_idx).unwrap()
  }
}


impl Game<State, Transient> for RunningGame {
  fn fresh(flags: ArgMatches) -> RunningGame {
    RunningGame::from_snapshot(State::new(), flags)
  }

  fn from_snapshot(state: State, flags: ArgMatches) -> RunningGame {
    let port = flags
      .value_of("port")
      .and_then(|v| u16::from_str(&v).ok())
      .unwrap();
    RunningGame::new(state, LiteServer::new(("0.0.0.0", port)))
  }

  fn from_opaque(state: State, network: LiteServer) -> RunningGame {
    RunningGame::new(state, network)
  }

  fn run(&mut self) {
    let now = PreciseTime::now();

    let delta = self.last_run_time.to(now.clone());

    self.last_run_time = now;
    let microsecond_delta = delta.num_microseconds()
      .expect("time between runs was way too long (over 280k years!)");
    let next_timestamp = self.state.get_time().get_timestamp().clone() + microsecond_delta;
    self.state.mut_time().set_timestamp(next_timestamp);

    self.update_network_state();
  }

  fn build_state(&self) -> State {
    let mut state = self.state.clone();
    state
  }

  fn build_transient(self) -> Transient {
    self.network
  }
}
