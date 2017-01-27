use protobuf::Message;
use fern::DispatchConfig;

pub fn get_default_logger_config(file_path: &str) -> DispatchConfig {
  DispatchConfig {
      format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
          // This is a fairly simple format, though it's possible to do more complicated ones.
          // This closure can contain any code, as long as it produces a String message.
          format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
      }),
      output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file(file_path)],
      level: log::LogLevelFilter::Trace,
  };
}

trait Game<P: Message, T> {
  fn new_from_flags(flags: ArgMatches) -> Self;
  fn new_from_hotload(persistent: P, transient: T) -> Self;
  fn copy_persistent_state(&self) -> P
  fn take_transient_state(self) -> T
}

struct DylibGame<P: Message, T, G: Game<P, T>> {
  flags: Option<ArgMatches>,
  game: Option<G>
}

impl <P: Message, T> DylibGame<S> {
  fn initialize_with_flags(matches: ArgMatches) -> DylibGame<P, T> {
    G::new_from_flags(matches)
  }

  fn initialize_with_prior_state(persistent: P, transient: T) -> {
  }

}

macro_rules! generate_ffi {
  (dylib: $ident) => {
    #[no_mangle]
    pub fn initialize_with_flags(matches: ArgMatches) {
      $dylib.lock().unwrap().initialize_with_flags(matches);
    }

    #[no_mangle]
    pub fn initialize_with_prior_state(state: *mut libc::c_void) {
      let dylib = $dylib.lock().unwrap();
      let mut opaque_state = unsafe { Box::from_raw(state as *mut ($game:: $game::T) };
      $dylib.lock().unwrap().initialize_with_prior_state(opaque_state);
    }

    #[no_mangle]
    pub fn run() {
      $dylib.lock().unwrap().run();
    }

    #[no_mangle]
    pub fn dump_state() -> *mut libc::c_void {
      let opaque_state = GAME_STATE.lock().unwrap().dump_state();
      Box::into_raw(Box::new(opaque_state)) as *mut libc::c_void
    }
  }
}
