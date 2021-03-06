use libc::{self, c_int};

#[macro_use]
pub mod dlsym;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod epoll;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::epoll::{Events, Selector};

#[cfg(any(target_os = "bitrig", target_os = "dragonfly",
    target_os = "freebsd", target_os = "ios", target_os = "macos",
    target_os = "netbsd", target_os = "openbsd"))]
mod kqueue;

#[cfg(any(target_os = "bitrig", target_os = "dragonfly",
    target_os = "freebsd", target_os = "ios", target_os = "macos",
    target_os = "netbsd", target_os = "openbsd"))]
pub use self::kqueue::{Events, Selector};

mod awakener;
mod eventedfd;
mod io;
mod tcp;
mod udp;
mod uds;
mod iovec;

pub use self::awakener::Awakener;
pub use self::eventedfd::EventedFd;
pub use self::io::{Io, set_nonblock};
pub use self::iovec::IoVec;
pub use self::tcp::{TcpStream, TcpListener};
pub use self::udp::UdpSocket;
pub use self::uds::UnixSocket;

use std::os::unix::io::FromRawFd;

pub fn pipe() -> ::io::Result<(Io, Io)> {
    // Use pipe2 for atomically setting O_CLOEXEC if we can, but otherwise
    // just fall back to using `pipe`.
    dlsym!(fn pipe2(*mut c_int, c_int) -> c_int);

    let mut pipes = [0; 2];
    let flags = libc::O_NONBLOCK | libc::O_CLOEXEC;
    unsafe {
        match pipe2.get() {
            Some(pipe2_fn) => {
                try!(cvt(pipe2_fn(pipes.as_mut_ptr(), flags)));
            }
            None => {
                try!(cvt(libc::pipe(pipes.as_mut_ptr())));
                libc::fcntl(pipes[0], libc::F_SETFL, flags);
                libc::fcntl(pipes[1], libc::F_SETFL, flags);
            }
        }
    }

    unsafe {
        Ok((Io::from_raw_fd(pipes[0]), Io::from_raw_fd(pipes[1])))
    }
}

trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

impl IsMinusOne for i32 {
    fn is_minus_one(&self) -> bool { *self == -1 }
}
impl IsMinusOne for isize {
    fn is_minus_one(&self) -> bool { *self == -1 }
}

fn cvt<T: IsMinusOne>(t: T) -> ::io::Result<T> {
    use std::io;

    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
