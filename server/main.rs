extern crate clap;

use std::str::FromStr;
use clap::{App, Arg, ArgMatches};

static EXAMPLE_COMMAND: &'static str = "space_coop -- -p 8888";

fn main() {
  let matches = App::new("space coop")
    .usage(EXAMPLE_COMMAND)
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .help("Server's port")
      .takes_value(true)
      .default_value("7090")
      .value_name("PORT"))
    .get_matches();

  // prototype2::server::start(port_from(&server_matches))
  println!("prospective port: {}", port_from(&matches));
}

fn port_from(matches: &ArgMatches) -> u16 {
  matches.value_of("port").and_then(|v| u16::from_str(&v).ok()).unwrap()
}
