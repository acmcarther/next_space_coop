//! A fast, low-level IO library for Rust focusing on non-blocking APIs, event
//! notification, and other useful utilities for building high performance IO
//! apps.
//!
//! # Goals
//!
//! * Fast - minimal overhead over the equivalent OS facilities (epoll, kqueue, etc...)
//! * Zero allocations
//! * A scalable readiness-based API, similar to epoll on Linux
//! * Design to allow for stack allocated buffers when possible (avoid double buffering).
//! * Provide utilities such as a timers, a notification channel, buffer abstractions, and a slab.
//!
//! # Usage
//!
//! Using mio starts by creating an [EventLoop](struct.EventLoop.html), which
//! handles receiving events from the OS and dispatching them to a supplied
//! [Handler](handler/trait.Handler.html).
//!
//! # Example
//!
//! ```
//! use mio::*;
//! use mio::tcp::{TcpListener, TcpStream};
//!
//! // Setup some tokens to allow us to identify which event is
//! // for which socket.
//! const SERVER: Token = Token(0);
//! const CLIENT: Token = Token(1);
//!
//! let addr = "127.0.0.1:13265".parse().unwrap();
//!
//! // Setup the server socket
//! let server = TcpListener::bind(&addr).unwrap();
//!
//! // Create an poll instance
//! let poll = Poll::new().unwrap();
//!
//! // Start listening for incoming connections
//! poll.register(&server, SERVER, Ready::readable(),
//!               PollOpt::edge()).unwrap();
//!
//! // Setup the client socket
//! let sock = TcpStream::connect(&addr).unwrap();
//!
//! // Register the socket
//! poll.register(&sock, CLIENT, Ready::readable(),
//!               PollOpt::edge()).unwrap();
//!
//! // Create storage for events
//! let mut events = Events::with_capacity(1024);
//!
//! loop {
//!     poll.poll(&mut events, None).unwrap();
//!
//!     for event in events.iter() {
//!         match event.token() {
//!             SERVER => {
//!                 // Accept and drop the socket immediately, this will close
//!                 // the socket and notify the client of the EOF.
//!                 let _ = server.accept();
//!             }
//!             CLIENT => {
//!                 // The server just shuts down the socket, let's just exit
//!                 // from our event loop.
//!                 return;
//!             }
//!             _ => unreachable!(),
//!         }
//!     }
//! }
//!
//! ```

#![doc(html_root_url = "https://docs.rs/mio/0.6.1")]
#![crate_name = "mio"]
#![cfg_attr(unix, deny(warnings))]

extern crate lazycell;
extern crate net2;
extern crate slab;

#[cfg(unix)]
extern crate nix;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate miow;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate kernel32;

#[macro_use]
extern crate log;

#[cfg(test)]
extern crate env_logger;

mod event;
mod io;
mod net;
mod poll;
mod sys;
mod token;

pub mod channel;
pub mod timer;

/// EventLoop and other deprecated types
pub mod deprecated;

pub use event::{
    PollOpt,
    Ready,
    Event,
};
pub use io::{
    Evented,
    would_block,
};
pub use net::{
    tcp,
    udp,
};
pub use poll::{
    Poll,
    Events,
    EventsIter,
    Registration,
    SetReadiness,
};
pub use token::{
    Token,
};

#[cfg(unix)]
pub mod unix {
    //! Unix only extensions

    pub use sys::{
        EventedFd,
    };
}

/// Windows-only extensions to the mio crate.
///
/// Mio on windows is currently implemented with IOCP for a high-performance
/// implementation of asynchronous I/O. Mio then provides TCP and UDP as sample
/// bindings for the system to connect networking types to asynchronous I/O. On
/// Unix this scheme is then also extensible to all other file descriptors with
/// the `EventedFd` type, but on Windows no such analog is available. The
/// purpose of this module, however, is to similarly provide a mechanism for
/// foreign I/O types to get hooked up into the IOCP event loop.
///
/// This module provides two types for interfacing with a custom IOCP handle:
///
/// * `Binding` - this type is intended to govern binding with mio's `Poll`
///   type. Each I/O object should contain an instance of `Binding` that's
///   interfaced with for the implementation of the `Evented` trait. The
///   `register`, `reregister`, and `deregister` methods for the `Evented` trait
///   all have rough analogs with `Binding`.
///
///   Note that this type **does not handle readiness**. That is, this type does
///   not handle whether sockets are readable/writable/etc. It's intended that
///   IOCP types will internally manage this state with a `SetReadiness` type
///   from the `poll` module. The `SetReadiness` is typically lazily created on
///   the first time that `Evented::register` is called and then stored in the
///   I/O object.
///
///   Also note that for types which represent streams of bytes the mio
///   interface of *readiness* doesn't map directly to the Windows model of
///   *completion*. This means that types will have to perform internal
///   buffering to ensure that a readiness interface can be provided. For a
///   sample implementation see the TCP/UDP modules in mio itself.
///
/// * `Overlapped` - this type is intended to be used as the concreate instances
///   of the `OVERLAPPED` type that most win32 methods expect. It's crucial, for
///   safety, that all asynchronous operations are initiated with an instance of
///   `Overlapped` and not another instantiation of `OVERLAPPED`.
///
///   Mio's `Overlapped` type is created with a function pointer that receives
///   a `OVERLAPPED_ENTRY` type when called. This `OVERLAPPED_ENTRY` type is
///   defined in the `winapi` crate. Whenever a completion is posted to an IOCP
///   object the `OVERLAPPED` that was signaled will be interpreted as
///   `Overlapped` in the mio crate and this function pointer will be invoked.
///   Through this function pointer, and through the `OVERLAPPED` pointer,
///   implementations can handle management of I/O events.
///
/// When put together these two types enable custom Windows handles to be
/// registered with mio's event loops. The `Binding` type is used to associate
/// handles and the `Overlapped` type is used to execute I/O operations. When
/// the I/O operations are completed a custom function pointer is called which
/// typically modifies a `SetReadiness` set by `Evented` methods which will get
/// later hooked into the mio event loop.
#[cfg(windows)]
pub mod windows {

    pub use sys::{Overlapped, Binding};
}

// Conversion utilities
mod convert {
    use std::time::Duration;

    const NANOS_PER_MILLI: u32 = 1_000_000;
    const MILLIS_PER_SEC: u64 = 1_000;

    /// Convert a `Duration` to milliseconds, rounding up and saturating at
    /// `u64::MAX`.
    ///
    /// The saturating is fine because `u64::MAX` milliseconds are still many
    /// million years.
    pub fn millis(duration: Duration) -> u64 {
        // Round up.
        let millis = (duration.subsec_nanos() + NANOS_PER_MILLI - 1) / NANOS_PER_MILLI;
        duration.as_secs().saturating_mul(MILLIS_PER_SEC).saturating_add(millis as u64)
    }
}
