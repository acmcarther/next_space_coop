use gaffer_udp::GafferPacket;
use gaffer_udp::non_blocking::GafferSocket;
use itertools::Itertools;
use lite_proto::lite::LiteMessage;
use lite_proto::lite::LiteMessage_Type;
use protobuf::Message;
use protobuf;
use protocol::ConnectionStatus;
use protocol::LiteServerEvent;
use protocol::ServerNegotiationStatus;
use protocol::WriteResult;
use protocol;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::time::SystemTime;
use clock::Clock;
use clock::SystemClock;

pub struct LiteServer {
  internal_socket: GafferSocket,
  negotiations: HashMap<SocketAddr, ServerNegotiationStatus>,
  connections: HashMap<SocketAddr, ConnectionStatus>,
  _internal_key_counter: u32,
  clock: Box<Clock>
}

impl LiteServer {
  pub fn new<T: ToSocketAddrs>(tsa: T) -> LiteServer {
    let mut socket_addrs = tsa.to_socket_addrs()
      .expect("couldn't coerce to sockets");
    let single_addr = socket_addrs.next()
      .expect("there was no mapping to addr");

    let gaffer_socket = GafferSocket::bind(single_addr)
      .expect("couldn't bind to that addr");

    trace!("Initialized server on {}", single_addr);
    LiteServer {
      internal_socket: gaffer_socket,
      negotiations: HashMap::new(),
      connections: HashMap::new(),
      _internal_key_counter: 1, // self is 0
      clock: Box::new(SystemClock),
    }
  }

  // TESTING_ONLY
  pub fn set_clock<T: 'static + Clock>(&mut self, c: T) {
    self.clock = Box::new(c);
  }

  fn syn_ack_bytes(&mut self, outgoing_id: u32) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::SYN_ACK);
    connect_msg.set_origin_id(0);
    connect_msg.set_set_your_id(outgoing_id);
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  fn data_bytes(&mut self, payload: Vec<u8>) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::SYN_ACK);
    connect_msg.set_origin_id(0);
    connect_msg.set_data(payload);
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  fn unknown_sender_bytes(&self) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::UNKNOWN_SENDER);
    connect_msg.set_origin_id(0);
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  pub fn read(&mut self) -> Vec<LiteServerEvent> {
    use protocol::ConnectionStatus::*;
    use protocol::ServerNegotiationStatus::*;

    let mut events = self.cull_dead_connections();

    while let Some(pkt) = self.internal_socket.recv().expect("broken gaffer pipe") {
      let source = pkt.addr;
      let payload = protobuf::parse_from_bytes::<LiteMessage>(&pkt.payload);

      let negotiation_status = self.negotiations.get(&source).cloned();
      let connection_status = self.connections.get(&source).cloned();

      let now = self.clock.now();
      match (payload, negotiation_status, connection_status) {
        (Err(error), _, _) => {
          // No payload, no problem
          trace!("Could not parse payload: {}", error);
        },
        // Re-SYN
        (Ok(ref msg), None, Some(Disconnected { id }))
            if msg.get_packet_type() == LiteMessage_Type::SYN => {
          let outgoing_msg = self.syn_ack_bytes(id);

          self.negotiations.insert(source.clone(), ClientSyn { last_message: now });
          self.connections.insert(source.clone(), Disconnected { id: id });
          self.internal_socket.send(GafferPacket::new(source, outgoing_msg));
        },
        // SYN
        (Ok(ref msg), None, None)
            if msg.get_packet_type() == LiteMessage_Type::SYN => {
          let outgoing_id = self._internal_key_counter;
          self._internal_key_counter = self._internal_key_counter + 1;
          let outgoing_msg = self.syn_ack_bytes(outgoing_id);

          self.negotiations.insert(source.clone(), ClientSyn { last_message: now });
          self.connections.insert(source.clone(), Disconnected { id: outgoing_id });
          self.internal_socket.send(GafferPacket::new(source, outgoing_msg));
        }
        // ACK
        (Ok(ref msg), Some(ClientSyn { last_message }), Some(Disconnected { id }))
            if msg.get_packet_type() == LiteMessage_Type::ACK
            && now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::SYN_ACK_WAIT_MS => {
          self.negotiations.remove(&source);
          self.connections.insert(source.clone(), Connected { id: id, last_message: now });
          events.push(LiteServerEvent::ConnectionEstablished { client_id: id });
        }
        // KEEP_ALIVE
        (Ok(ref msg), _, Some(Connected { id, last_message }))
            if msg.get_packet_type() == LiteMessage_Type::KEEP_ALIVE
            && now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::SYN_ACK_WAIT_MS => {
          self.connections.insert(source.clone(), Connected { id: id, last_message: now });
        }
        // CLOSE
        (Ok(ref msg), _, Some(Connected { id, last_message }))
            if msg.get_packet_type() == LiteMessage_Type::CLOSE
            && now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::SYN_ACK_WAIT_MS => {
          self.connections.insert(source.clone(), Disconnected { id: id });
          events.push(LiteServerEvent::ConnectionDropped_Disconnect { client_id: id });
        }
        // UNKNOWN_SENDER
        (Ok(ref msg), _, Some(Connected { id, last_message }))
            if msg.get_packet_type() == LiteMessage_Type::UNKNOWN_SENDER
            && now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::SYN_ACK_WAIT_MS => {
          self.connections.insert(source.clone(), Disconnected { id: id });
        }
        // DATA
        (Ok(ref mut msg), _, Some(Connected { id, last_message }))
            if msg.get_packet_type() == LiteMessage_Type::DATA
            && now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::SYN_ACK_WAIT_MS => {
          self.connections.insert(source.clone(), Connected { id: id , last_message: now });

          events.push(LiteServerEvent::Data { client_id: id, bytes: msg.take_data() })
        }
        // Unknown sender to us, give em the ol' UNKNOWN_SENDER
        (Ok(ref msg), None, None) => {
          let outgoing_msg = self.unknown_sender_bytes();
          self.internal_socket.send(GafferPacket::new(source, outgoing_msg));
        }
        _ => {}
      }
    }

    events
  }

  pub fn write(&mut self, client_id: u32, bytes: Vec<u8>) -> WriteResult {
    let address = self.connections.iter()
      .find(|&(k, v)| v.get_id() == client_id && v.is_connected())
      .map(|(k, _)| k.clone());

    match address {
      Some(addr) => {
        let outgoing_msg = self.data_bytes(bytes);
        self.internal_socket.send(GafferPacket::new(addr, outgoing_msg));
        WriteResult::Success
      },
      None => {
        WriteResult::Failure_NotConnected
      }
    }
  }

  fn cull_dead_connections(&mut self) -> Vec<LiteServerEvent> {
    use protocol::ConnectionStatus::*;
    let now = self.clock.now();

    let expired_connections = self.connections.iter()
      .filter(|&(_, value)| match value {
        &Connected { id, last_message } => {
          now.duration_since(last_message)
            .map(|d| d.as_secs() as u32 > protocol::CONNECTION_EXPIRY_TIME_S)
            .unwrap_or(false)
        },
        _ => { false }
      })
      .map(|(socket_addr, _)| {
        socket_addr.clone()
      })
      .collect::<Vec<SocketAddr>>();

    expired_connections.into_iter().map(|addr| {
      let old_val = self.connections.remove(&addr).expect("expected to remove a non-existent connection");
      // Toss old value in the trash
      self.connections.insert(addr, ConnectionStatus::Disconnected { id: old_val.get_id() });
      LiteServerEvent::ConnectionDropped_Lag { client_id: old_val.get_id() }
    }).collect::<Vec<LiteServerEvent>>()
  }
}
