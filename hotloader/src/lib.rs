extern crate clap;
extern crate hotload;

use clap::ArgMatches;
use hotload::BasicProxy;
use hotload::Hotloader;
use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub fn run(matches: ArgMatches) {
  let dylib_path = dylib_from(&matches);
  println!("current_dir:{:?}", env::current_dir());
  println!("dylib_path:{:?}", PathBuf::from(dylib_path.clone()));
  let mut hotloader = Hotloader::<BasicProxy>::new(PathBuf::from(dylib_path));
  hotloader.get_proxy().unwrap().set_flags(matches);
  loop {
    hotloader.get_proxy().unwrap().run();
    thread::sleep(Duration::from_millis(200));
  }
}

fn dylib_from(matches: &ArgMatches) -> String {
  matches.value_of("dylib_path").unwrap().to_owned()
}
