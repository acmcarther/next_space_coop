use clap::ArgMatches;
use gameplay::Gameplay;
use itertools::Itertools;
use lite::LiteServer;
use lite::LiteServerEvent;
use lite;
use protobuf::Message;
use loadable::Game;
use lobby::Lobby;
use game_proto::game::PlayerLike;
use game_proto::game::Snapshot;
use network_proto::network::ClientMessage;
use network_proto::network::ServerMessage;
use network_proto::network::ServerMessage_MessageType;
use protobuf;
use protobuf::RepeatedField;
use game_proto::game::GameMode;
use state_proto::state::NetworkPlayer;
use state_proto::state::Mailbox;
use state_proto::state::PlayerData;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::thread;
use time::Duration;
use time::SteadyTime;

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
  network: LiteServer,
  last_sim_time: SteadyTime,
  last_net_time: SteadyTime,
  next_sim_time: SteadyTime,
  next_net_time: SteadyTime,
}

impl RunningGame {
  fn new(state: State, network: LiteServer) -> RunningGame {
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
    let mut incoming_msgs = self.network.read();

    while !incoming_msgs.is_empty() {
      self.process_messages(incoming_msgs);
      incoming_msgs = self.network.read();
    }
    let player_likes = self.produce_playerlikes();
    let network = &mut self.network;

    self.state.mut_players().iter_mut().foreach(|mut player| {
      let id = player.get_id();

      // Produce state snapshot for clients
      let mut snapshot = Snapshot::new();
      snapshot.set_your_id(id);
      snapshot.set_players(RepeatedField::<PlayerLike>::from_vec(player_likes.clone()));
      let mut snapshot_msg = ServerMessage::new();
      snapshot_msg.set_message_type(ServerMessage_MessageType::SNAPSHOT);
      snapshot_msg.set_snapshot(snapshot);
      player.mut_mailbox().mut_outbound().push(snapshot_msg);

      // Send all outbound msgs
      player.mut_mailbox().take_outbound().into_iter().foreach(|message| {
        let payload_bytes = message.write_to_bytes().expect("message wasnt serialzable?");
        if let lite::WriteResult::Failure_NotConnected = network.write(id, payload_bytes) {
          trace!("Oops, tried to write a message to {}, who is disconnected.", id);
          player.set_connected(false);
        }
      })
    });
  }

  fn produce_playerlikes(&self) -> Vec<PlayerLike> {
    self.state.get_game().get_player_data().iter().map(|p| {
      let mut player_like = PlayerLike::new();
      player_like.set_id(p.get_id());
      player_like.set_ready(p.get_ready());
      player_like.set_match_data(p.get_match_data().clone());
      player_like
    }).collect()
  }

  fn bridge_network_state_and_game_state(&mut self) {
    // Push events onto the associated players
    let mut mail_per_player = self.state.mut_players()
      .iter_mut()
      .filter(|net_player| net_player.has_mailbox())
      .map(|net_player| (net_player.get_id().clone(), net_player.take_mailbox()))
      .collect::<HashMap<u32, Mailbox>>();
    self.state.mut_game().mut_player_data().iter_mut().foreach(|player| {
      let player_id = player.get_network_player_id();
      if let Some(mailbox) = mail_per_player.remove(&player_id) {
        if !player.get_mailbox().get_inbound().is_empty() {
          warn!("game ply {} dropped some inbound mail on the ground", player_id)
        }
        if !player.get_mailbox().get_outbound().is_empty() {
          warn!("game ply {} dropped some outbound mail on the ground", player_id)
        }
        player.set_mailbox(mailbox);
      }
    });

    // Identify active network players
    let mut net_player_ids = self.state.get_players().iter()
      .filter(|ply| ply.get_connected())
      .map(|ply| ply.get_id().clone())
      .collect::<HashSet<u32>>();
    // Update game players active state from network players
    self.state.mut_game().mut_player_data().iter_mut()
      .foreach(|mut game_ply| {
        if net_player_ids.remove(&game_ply.get_network_player_id()) {
          game_ply.set_active(true);
        } else {
          game_ply.set_active(false);
        }
      });
    // Insert new network players as necessary
    net_player_ids.into_iter().foreach(|id| {
      let mut new_player_data = PlayerData::new();
      new_player_data.set_active(true);
      new_player_data.set_network_player_id(id);
      self.state.mut_game().mut_player_data().push(new_player_data);
    });
  }

  fn update_game_state(&mut self, microsecond_delta: i64) {
    // NOTE: if this borrow affects further usage of state, whatever you're doing probably should
    // not be in this function.
    let mut game = self.state.mut_game();
    let current_mode = game.get_mode().clone();

    match current_mode {
      GameMode::UNKNOWN => {
        game.set_mode(GameMode::LOBBY)
      },
      GameMode::LOBBY => {
        Lobby::new(&mut game).tick();
      }
      GameMode::PLAYING => {
        Gameplay::new(&mut game, microsecond_delta).tick();
      }
    }
  }

  fn bridge_game_state_and_network_state(&mut self) {
    // Push events onto the associated players
    let mut mail_per_player = self.state.mut_game().mut_player_data()
      .iter_mut()
      .filter(|game_player| game_player.has_mailbox())
      .map(|game_player| (game_player.get_network_player_id().clone(), game_player.take_mailbox()))
      .collect::<HashMap<u32, Mailbox>>();
    self.state.mut_players().iter_mut().foreach(|player| {
      let player_id = player.get_id();
      if let Some(mailbox) = mail_per_player.remove(&player_id) {
        if !player.get_mailbox().get_inbound().is_empty() {
          warn!("net ply {} dropped some inbound mail on the ground", player_id)
        }
        if !player.get_mailbox().get_outbound().is_empty() {
          warn!("net ply {} dropped some outbound mail on the ground", player_id)
        }
        player.set_mailbox(mailbox);
      }
    });
  }

  fn process_messages(&mut self, events: Vec<LiteServerEvent>) {
    use lite::LiteServerEvent::*;

    for event in events.into_iter() {
      match event {
        ConnectionEstablished { client_id } => {
          let player = self.find_or_establish_player(client_id);
          player.set_connected(true);
          let mut response = ServerMessage::new();
          response.set_message_type(ServerMessage_MessageType::CONNECTED);
          player.mut_mailbox().mut_outbound().push(response);
        }
        ConnectionDropped_Disconnect { client_id } | ConnectionDropped_Lag { client_id } => {
          let player = self.find_or_establish_player(client_id);
          player.set_connected(false);
          let mut response = ServerMessage::new();
          response.set_message_type(ServerMessage_MessageType::DISCONNECTED);
          player.mut_mailbox().mut_outbound().push(response);
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

          player.mut_mailbox().mut_inbound().push(message_res.unwrap());
        }
      }
    }
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

      let next_timestamp = self.state.get_time().get_timestamp().clone() + microsecond_delta;
      self.state.mut_time().set_timestamp(next_timestamp);

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
    self.network
  }
}
