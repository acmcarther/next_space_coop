extern crate clap;
extern crate hotload;

use clap::{App, Arg, ArgMatches};
use hotload::BasicProxy;
use hotload::Hotloader;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

static EXAMPLE_COMMAND: &'static str = "space_coop -- -d ./core/libcore.so";

fn main() {
  let matches = App::new("space coop")
    .usage(EXAMPLE_COMMAND)
    .arg(Arg::with_name("dylib_path")
      .short("d")
      .long("dylib")
      .default_value("./bazel-bin/space_coop/server/core/libcore.so")
      .help("Path to dylib")
      .takes_value(true))
    .get_matches();

  // prototype2::server::start(port_from(&server_matches))
  //let port = port_from(&matches);
  //println!("prospective port: {}", port_from(&matches));
  //let core = Core::new_with_port(port);
  //core.run();

  let dylib_path = dylib_from(&matches);
  println!("current_dir:{:?}", env::current_dir());
  println!("dylib_path:{:?}", PathBuf::from(dylib_path.clone()));
  let mut hotloader = Hotloader::<BasicProxy>::new(PathBuf::from(dylib_path));
  loop {
    hotloader.get_proxy().unwrap().run();
    thread::sleep(Duration::from_millis(200));
  }
}

fn port_from(matches: &ArgMatches) -> u16 {
  matches.value_of("port").and_then(|v| u16::from_str(&v).ok()).unwrap()
}

fn dylib_from(matches: &ArgMatches) -> String {
  matches.value_of("dylib_path").unwrap().to_owned()
}
