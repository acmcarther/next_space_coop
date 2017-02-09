use std::time::SystemTime;

pub const SYN_ACK_WAIT_MS: u32 = 500;
pub const SYN_ACK_RETRIES: u32 = 5;
pub const CONNECTION_EXPIRY_TIME_S: u32 = 5;
pub const PAYLOAD_MAX_SIZE: usize = 512;

pub enum WriteResult {
  Success,
  Failure_NotConnected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientNegotiationStatus {
  ClientSyn,
  ServerSynAck,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerNegotiationStatus {
  ClientSyn { last_message: SystemTime },
}

#[derive(Clone, PartialEq)]
pub enum ConnectionStatus {
  Disconnected { id: u32 },
  Connected { id: u32, last_message: SystemTime }
}

impl ConnectionStatus {
  pub fn get_id(&self) -> u32 {
    match self {
      &ConnectionStatus::Disconnected { id } => id,
      &ConnectionStatus::Connected { id, last_message } => id
    }
  }
}

impl ConnectionStatus {
  pub fn is_connected(&self) -> bool {
    match self {
      &ConnectionStatus::Disconnected { id } => false,
      &ConnectionStatus::Connected { id, last_message} => true
    }
  }
}

pub enum LiteServerEvent {
  ConnectionEstablished { client_id: u32 },
  ConnectionDropped_Lag { client_id: u32 },
  ConnectionDropped_Disconnect { client_id: u32 },
  Data { client_id: u32, bytes: Vec<u8> }
}

pub enum LiteClientEvent {
  ConnectionEstablished,
  ConnectionDropped_Lag,
  ConnectionDropped_Disconnect,
  Data { bytes: Vec<u8> }
}
