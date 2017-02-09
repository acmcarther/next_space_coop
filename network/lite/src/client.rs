use gaffer_udp::GafferPacket;
use gaffer_udp::non_blocking::GafferSocket;
use lite_proto::lite::LiteMessage;
use lite_proto::lite::LiteMessage_Type;
use protobuf::Message;
use protobuf;
use protocol::ClientNegotiationStatus;
use protocol::ConnectionStatus;
use protocol::LiteClientEvent;
use protocol::WriteResult;
use protocol;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::time::SystemTime;
use clock::Clock;
use clock::SystemClock;

pub struct LiteClient {
  internal_socket: GafferSocket,
  server: SocketAddr,
  negotiation: Option<ClientNegotiationStatus>,
  connection: ConnectionStatus,
  clock: Box<Clock>
}

impl LiteClient {
  pub fn new<T: ToSocketAddrs>(tsa_self: T, tsa_server: T) -> LiteClient {
    let client_addr = {
      let mut socket_addrs = tsa_self.to_socket_addrs()
        .expect("couldn't coerce to sockets");

      socket_addrs.next().expect("there was no mapping to addr")
    };

    let gaffer_socket = GafferSocket::bind(client_addr.clone())
        .expect("couldn't bind to that addr");

    let server_addr = {
      let mut socket_addrs = tsa_server.to_socket_addrs()
        .expect("couldn't coerce to sockets");

      socket_addrs.next().expect("there was no mapping to addr")
    };

    trace!("Initialized client on {}", client_addr);
    trace!("Directed at server on {}", server_addr);
    LiteClient {
      internal_socket: gaffer_socket,
      server: server_addr,
      negotiation: None,
      connection: ConnectionStatus::Disconnected { id: 0 },
      clock: Box::new(SystemClock),
    }
  }

  // TESTING_ONLY
  pub fn set_clock<T: 'static + Clock>(&mut self, c: T) {
    self.clock = Box::new(c);
  }

  fn syn_bytes() -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::SYN);
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  fn ack_bytes(our_id: u32) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::ACK);
    connect_msg.set_origin_id(our_id);
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  fn data_bytes(our_id: u32, payload: Vec<u8>) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_origin_id(our_id);
    connect_msg.set_packet_type(LiteMessage_Type::DATA);
    connect_msg.set_data(payload.to_vec());
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  fn unknown_sender_bytes(&self) -> Vec<u8> {
    let mut connect_msg = LiteMessage::new();
    connect_msg.set_packet_type(LiteMessage_Type::UNKNOWN_SENDER);
    connect_msg.set_origin_id(self.connection.get_id());
    connect_msg.write_to_bytes().expect("packet wasn't serializable")
  }

  // TODO(acmcarther): Proper error handling with results here.
  pub fn connect(&mut self) -> bool {
    use protocol::ConnectionStatus::*;
    use protocol::ClientNegotiationStatus::*;

    self.cull_dead_connections();
    let mut tries: u32 = 0;
    let mut failed = false;
    let mut deadline = self.clock.now()
      + Duration::from_millis(protocol::SYN_ACK_WAIT_MS as u64);

    trace!("attempting connect");
    let mut now = self.clock.now();
    while !failed
        && !self.connection.is_connected() {
      while now < deadline
          && !self.connection.is_connected() {
        let possible_lite_msg = self.internal_socket.recv()
          .expect("broken gaffer pipe")
          .and_then(|pkt| if pkt.addr == self.server { Some(pkt) } else { None })
          .and_then(|pkt| protobuf::parse_from_bytes::<LiteMessage>(&pkt.payload).ok());

        if possible_lite_msg.is_some() {
          trace!("matching: {:?}, {:?}, {:?}",
               self.negotiation, possible_lite_msg, tries);
        }
        match (&self.negotiation, possible_lite_msg, tries) {
          (_, _, protocol::SYN_ACK_RETRIES) => {
            failed = true;
          },
          // Send Syn
          (&None, _, _) => {
            trace!("sending pkt containing {:?}", LiteClient::syn_bytes());
            self.internal_socket.send(GafferPacket::new(self.server.clone(), LiteClient::syn_bytes()));
            self.negotiation = Some(ClientNegotiationStatus::ClientSyn);
            tries += 1;
          },
          // Respond to Syn+Ack
          (&Some(ClientSyn), Some(ref lite_msg), _)
              if lite_msg.get_packet_type() == LiteMessage_Type::SYN_ACK
              && lite_msg.get_set_your_id() != 0 => {
            let our_id = lite_msg.get_set_your_id();
            self.internal_socket.send(GafferPacket::new(self.server.clone(), LiteClient::ack_bytes(our_id)));
            self.negotiation = None;
            self.connection = Connected { id: our_id, last_message: now };
          },
          (_, None, _) => {
            // Nothin to do, no packet
          }
          _ => {
            // Some weird packets or something
          }
        }

        now = self.clock.now();
      }
      self.negotiation = None;
      deadline = self.clock.now()
        + Duration::from_millis(protocol::SYN_ACK_WAIT_MS as u64);
    }

    if failed {
      self.negotiation = None;
      self.connection = ConnectionStatus::Disconnected { id: 0 }
    }

    !failed
  }

  pub fn read(&mut self) -> Vec<LiteClientEvent> {
    let mut events = Vec::new();
    while let Some(pkt) = self.internal_socket.recv().expect("broken gaffer pipe") {
      let source = pkt.addr;
      let res_payload = protobuf::parse_from_bytes::<LiteMessage>(&pkt.payload);

      // Drop unparsable payloads
      if res_payload.is_err() {
        continue
      }

      let mut payload = res_payload.unwrap();

      let now = self.clock.now();
      match (payload.get_packet_type(), source == self.server) {
        // Someone random sent us a message, ignore it to prevent a potential cycle with server
        (_, false) => {},
        (LiteMessage_Type::UNKNOWN_SENDER, true) => {
          let own_id = self.connection.get_id();
          self.connection = ConnectionStatus::Disconnected { id: own_id };
          events.push(LiteClientEvent::ConnectionDropped_Lag)
        }
        (LiteMessage_Type::CLOSE, true) => {
          let own_id = self.connection.get_id();
          self.connection = ConnectionStatus::Disconnected { id: own_id };
          events.push(LiteClientEvent::ConnectionDropped_Disconnect)
        },
        (LiteMessage_Type::DATA, true) => {
          let own_id = self.connection.get_id();
          self.connection = ConnectionStatus::Connected { id: own_id , last_message: now };
          events.push(LiteClientEvent::Data { bytes: payload.take_data() })
        },
        // Some other packet we dont really care about, update last_message though
        (_, true) => {
          let own_id = self.connection.get_id();
          self.connection = ConnectionStatus::Connected { id: own_id , last_message: now }
        },
      }
    }

    events
  }

  pub fn write(&mut self, bytes: Vec<u8>) -> WriteResult {
    if !self.connect() {
      WriteResult::Failure_NotConnected
    } else {
      // Unenforced assert: get_id should return a real id if we're connected.
      self.internal_socket.send(GafferPacket::new(
          self.server.clone(),
          LiteClient::data_bytes(self.connection.get_id(), bytes)));
      WriteResult::Success
    }
  }

  fn cull_dead_connections(&mut self) {
    use protocol::ConnectionStatus::*;
    let now = self.clock.now();

    match self.connection {
      Connected { last_message, id }
          if now.duration_since(last_message).map(|d| d.as_secs() as u32).unwrap_or(0) > protocol::CONNECTION_EXPIRY_TIME_S => {
        self.connection = Disconnected { id: id };
      },
      _ => {}
    }
  }

}
