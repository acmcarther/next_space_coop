extern crate libc;
extern crate libloading;
extern crate clap;

#[cfg(unix)]
use libloading::os::unix::Symbol;
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use libloading::os::windows::Symbol;
// TODO: will not compile on windows for lack of file size

use libloading::Library;
use clap::ArgMatches;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time;
use std::fs;
use std::thread;
use std::time::Duration;

pub trait HotloadProxy {
  fn new(lib: Library, matches: ArgMatches) -> Self;
  fn hotload(lib: Library, state: *mut libc::c_void) -> Self;
  fn dump_state(self) -> *mut libc::c_void;
}

/** A basic proxy proxying "initialize", "run", and "dump_state" to a dylib */
pub struct BasicProxy {
  lib: Library,
  run_fn: Option<Symbol<fn()>>,
}

impl BasicProxy {
  /** Run the dylib's run function using the cached symbol */
  pub fn run(&mut self) {
    self.run_fn.as_mut().unwrap()();
  }
}

impl HotloadProxy for BasicProxy {
  /** Build a brand new proxy from a library */
  fn new(lib: Library, matches: ArgMatches) -> BasicProxy {
    // Initialize the library
    unsafe { lib.get::<fn(ArgMatches)>("new".as_bytes()).unwrap()(matches) }

    BasicProxy {
      run_fn: Some(unsafe { lib.get::<fn()>("run".as_bytes()).unwrap().into_raw() }),
      lib: lib
    }
  }

  /** Build a brand proxy from a prior opaque state */
  fn hotload(lib: Library, state: *mut libc::c_void) -> BasicProxy {
    // Hotload the library
    unsafe { lib.get::<fn(*mut libc::c_void)>("hotload".as_bytes()).unwrap()(state) }

    BasicProxy {
      run_fn: Some(unsafe { lib.get::<fn()>("run".as_bytes()).unwrap().into_raw() }),
      lib: lib
    }
  }

  /** Dump the opaque state from the contained dylib */
  fn dump_state(mut self) -> *mut libc::c_void {
    drop(self.run_fn.take()); // ensure run_fn is dropped before contained lib
    unsafe {
      self.lib.get::<fn() -> *mut libc::c_void>("dump_state".as_bytes()).unwrap()()
    }
  }
}

/** A struct containing a hotloadable library object, and a user defined method proxy */
pub struct Hotloader<T: HotloadProxy> {
  proxy: Option<T>,
  watched_path: PathBuf,
  last_modified: SystemTime,
  last_size: u64,
}

impl <T: HotloadProxy> Hotloader<T> {
  /** Build a fresh hotloader from a path to a dylib */
  pub fn new(dylib_path: PathBuf, matches: ArgMatches) -> Hotloader<T> {
    let mut hotloader = Hotloader {
      proxy: None,
      watched_path: dylib_path,
      last_modified: time::UNIX_EPOCH,
      last_size: 0,
    };

    hotloader.initialize(matches);
    hotloader.try_hotload();
    hotloader
  }

  fn initialize(&mut self, matches: ArgMatches) {
    let (new_size, new_last_modified) = self.lib_metadata().expect("Could not find dylib");
    let lib = self.load_lib();

    self.proxy = Some(T::new(lib, matches));
    self.last_modified = new_last_modified;
    self.last_size = new_size;
  }

  fn lib_metadata(&self) -> Option<(u64, SystemTime)> {
    fs::metadata(self.watched_path.clone())
      .and_then(|v| v.modified().map(|time| (v.size(), time)))
      .ok()
  }

  fn load_lib(&self) -> Library {
    let mut lib = None;
    while lib.is_none() {
      lib = Library::new(self.watched_path.clone()).ok();
      if lib.is_none() {
        thread::sleep(Duration::from_millis(50));
      }
    }

    lib.unwrap()
  }

  /** Get the *optional* method proxy. It may not be present if hotloading fails */
  pub fn get_proxy(&mut self) -> Option<&mut T> {
    self.try_hotload();
    self.proxy.as_mut()
  }

  /** Attempt a hotload or do nothing */
  fn try_hotload(&mut self) {
    let (new_size, new_last_modified) = self.lib_metadata().unwrap_or((self.last_size, self.last_modified));
    if new_last_modified <= self.last_modified || new_size == 0 {
      return;
    }

    // Take state from proxy and drop dylib (so we can reload it)
    // see docs on dlopen for why dropping is necessary
    let last_state = self.proxy.take().map(|c| c.dump_state())
      .expect("tried to hotload an uninitialized library");

    self.proxy = Some(T::hotload(self.load_lib(), last_state));
    self.last_modified = new_last_modified;
    self.last_size = new_size;
  }
}
