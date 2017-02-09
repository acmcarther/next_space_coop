use lite::client::LiteClient;
use lite::server::LiteServer;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;
use log;
use fern;
use time;
use clock::MockClock;

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

#[test]
fn lite_client_can_start_without_exploding() {
  let _ = LiteClient::new("127.0.0.1:27001", "127.0.0.1:27002");
}

#[test]
fn lite_client_connect_to_a_server() {
  init_debug_logger();
  let mut server = LiteServer::new("127.0.0.1:27003");
  let mut client = LiteClient::new("127.0.0.1:27004", "127.0.0.1:27003");

  let should_exit = Arc::new(Mutex::new(false));
  let should_exit_t = should_exit.clone();

  thread::spawn(move || {
    while !(*should_exit_t.lock().unwrap()) {
      server.read();
      thread::sleep(Duration::from_millis(10));
    }
  });

  let try_connect = client.connect();
  *should_exit.lock().unwrap() = true;

  assert!(try_connect, "expected connection to be established")
}
