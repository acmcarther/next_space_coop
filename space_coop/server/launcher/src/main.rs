extern crate clap;
extern crate launcher;

use clap::App;

static EXAMPLE_COMMAND: &'static str = "space_coop -- -d ./core/libcore.so";

fn main() {
  launcher::run(
      App::new("space coop").usage(EXAMPLE_COMMAND),
      "./bazel-bin/space_coop/server/core/libcore.so".to_owned());
}
