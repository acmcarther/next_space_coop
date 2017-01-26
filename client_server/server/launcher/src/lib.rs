extern crate clap;
extern crate hotloader;

use clap::App;
use clap::Arg;

pub fn run(application: App, dylib_path: String) {
  let matches = application
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .default_value("8829")
      .help("Port to host on")
      .takes_value(true))
    .arg(Arg::with_name("dylib_path")
      .short("d")
      .long("dylib")
      .default_value(&dylib_path)
      .help("Path to dylib")
      .takes_value(true))
    .get_matches();

  hotloader::run(matches);
}
