// TODO(acmcarther): This is copy pasta from space_coop/server
extern crate clap;
extern crate loader;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use loader::BasicProxy;
use loader::Hotloader;
use std::env;
use std::path::PathBuf;

static EXAMPLE_COMMAND: &'static str = "space_coop -- -c 0.0.0.0:9999 -s 192.168.0.1:8888";

fn dylib_from(matches: &ArgMatches) -> String {
  matches.value_of("dylib_path").unwrap().to_owned()
}

pub fn main() {
  let matches = App::new("Space Coop Client")
    .usage(EXAMPLE_COMMAND)
    .arg(Arg::with_name("client address")
      .short("c")
      .long("client_address")
      .help("Client's address")
      .takes_value(true)
      .default_value("0.0.0.0:7091")
      .value_name("ADDRESS:PORT"))
    .arg(Arg::with_name("server address")
      .short("s")
      .help("Server's address and port")
      .long("server_address")
      .value_name("ADDRESS:PORT")
      .takes_value(true)
      .default_value("127.0.0.1:7090"))
    .arg(Arg::with_name("dylib_path")
      .short("d")
      .long("dylib")
      // Works if launcher is launched from project root as
      // ./bazel-bin/space_coop/client/launcher/launcher
      .default_value("./bazel-bin/space_coop/client/core/libcore.so")
      .help("Path to dylib")
      .takes_value(true))
    .get_matches();

  let dylib_path = PathBuf::from(dylib_from(&matches));
  println!("current_dir:{:?}", env::current_dir());
  println!("dylib_path:{:?}", dylib_path);
  let mut hotloader = Hotloader::<BasicProxy>::new(dylib_path, matches);
  loop {
    hotloader.get_proxy().unwrap().run();
  }
}
