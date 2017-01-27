extern crate clap;
extern crate hotload;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use hotload::BasicProxy;
use hotload::Hotloader;
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

static EXAMPLE_COMMAND: &'static str = "space_coop -- -d ./core/libcore.so";

fn dylib_from(matches: &ArgMatches) -> String {
  matches.value_of("dylib_path").unwrap().to_owned()
}

fn main() {
  let matches = App::new("space coop server")
    .usage(EXAMPLE_COMMAND)
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .default_value("8829")
      .help("Port to host on")
      .takes_value(true))
    .arg(Arg::with_name("dylib_path")
      .short("d")
      .long("dylib")
      // Works if launcher is launched from project root as
      // ./bazel-bin/space_coop/server/launcher/launcher
      .default_value("./bazel-bin/space_coop/server/core/libcore.so")
      .help("Path to dylib")
      .takes_value(true))
    .get_matches();

  let dylib_path = PathBuf::from(dylib_from(&matches));
  println!("current_dir:{:?}", env::current_dir());
  println!("dylib_path:{:?}", dylib_path);
  let mut hotloader = Hotloader::<BasicProxy>::new(dylib_path, matches);
  loop {
    hotloader.get_proxy().unwrap().run();
    thread::sleep(Duration::from_millis(200));
  }
}
