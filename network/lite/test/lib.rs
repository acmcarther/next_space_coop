extern crate lite;
extern crate lite_proto;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate protobuf;
extern crate gaffer_udp;
extern crate time;
extern crate fern;
#[macro_use]
extern crate log;
extern crate clock;

use clock::MockClock;
use lite::LiteClient;
use lite::LiteServer;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

lazy_static! {
  pub static ref MAKE_LOGGER_ONCE: () = {
    init_debug_logger();
    ()
  };
}

pub fn init() {
  MAKE_LOGGER_ONCE.clone()
}

fn init_debug_logger() {
  let logger_config = fern::DispatchConfig {
    format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
      format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
    }),
    output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("output.log")],
    level: log::LogLevelFilter::Info,
  };
  fern::init_global_logger(logger_config, log::LogLevelFilter::Info).unwrap();
}

fn spin_off_server_on_port(port: u16) -> Sender<()> {
  let (tx, rx) = mpsc::channel();
  let mut server = LiteServer::new(("127.0.0.1", port));

  thread::spawn(move || {
    while rx.try_recv() != Err(TryRecvError::Disconnected) {
      server.read();
      thread::sleep(Duration::from_millis(10));
    }
  });

  return tx;
}

#[test]
fn lite_server_can_start_without_exploding() {
  let client = LiteServer::new("127.0.0.1:27000");
}

#[test]
fn lite_client_can_start_without_exploding() {
  ::init();
  let _ = LiteClient::new("127.0.0.1:27001", "127.0.0.1:27002");
}

#[test]
#[allow(unused_variables)]
fn lite_client_connect_to_a_server() {
  ::init();
  let drop_guard = spin_off_server_on_port(27003);
  let mut client = LiteClient::new("127.0.0.1:27004", "127.0.0.1:27003");

  let try_connect = client.connect();

  assert!(try_connect, "expected connection to be established")
}
