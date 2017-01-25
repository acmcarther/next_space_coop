extern crate clap;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use std::net::ToSocketAddrs;
use std::str::FromStr;

static EXAMPLE_COMMAND: &'static str = "space_coop -- -p 9999 -s 192.168.0.1:8888";

pub fn main() {
  let matches = App::new("Space Coop Client")
    .usage(EXAMPLE_COMMAND)
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .help("Client's port")
      .takes_value(true)
      .default_value("7190")
      .value_name("PORT"))
    .arg(Arg::with_name("server address")
      .short("s")
      .help("Server's address and port")
      .long("server_address")
      .value_name("ADDRESS:PORT")
      .takes_value(true)
      .default_value("127.0.0.1:7090")
      .required(true))
    .get_matches();

  // prototype2::client::start(port_from(&matches), addr_from(&matches))
  println!("prospective port: {}", port_from(&matches));
  println!("prospective addr: {}", addr_from(&matches));
}

fn port_from(matches: &ArgMatches) -> u16 {
  matches.value_of("port").and_then(|v| u16::from_str(&v).ok()).unwrap()
}

fn addr_from(matches: &ArgMatches) -> std::net::SocketAddr {
  matches.value_of("server address")
    .and_then(|v| v.to_socket_addrs().ok())
    .and_then(|mut socket_addr_iter| socket_addr_iter.next())
    .unwrap()
}
