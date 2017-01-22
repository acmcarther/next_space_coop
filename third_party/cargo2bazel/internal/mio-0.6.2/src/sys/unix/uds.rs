use {io, Evented, Ready, Poll, PollOpt, Token};
use sys::unix::{net, nix, Io};
use std::io::{Read, Write};
use std::path::Path;
use std::os::unix::io::{RawFd, IntoRawFd, AsRawFd, FromRawFd};
use nix::sys::socket::MsgFlags;
pub use nix::sys::socket::Shutdown;

#[derive(Debug)]
pub struct UnixSocket {
    io: Io,
}

impl UnixSocket {
    /// Returns a new, unbound, non-blocking Unix domain socket
    pub fn stream() -> io::Result<UnixSocket> {
        UnixSocket::new(nix::SockType::Stream)
    }

    fn new(ty: nix::SockType) -> io::Result<UnixSocket> {
        let fd = try!(net::socket(nix::AddressFamily::Unix, ty, true));
        Ok(From::from(unsafe { Io::from_raw_fd(fd) }))
    }

    /// Connect the socket to the specified address
    pub fn connect<P: AsRef<Path> + ?Sized>(&self, addr: &P) -> io::Result<bool> {
        net::connect(&self.io, &try!(to_nix_addr(addr)))
    }

    /// Listen for incoming requests
    pub fn listen(&self, backlog: usize) -> io::Result<()> {
        net::listen(&self.io, backlog)
    }

    pub fn accept(&self) -> io::Result<UnixSocket> {
        net::accept(&self.io, true)
            .map(|fd| From::from(unsafe { Io::from_raw_fd(fd) }))
    }

    /// Bind the socket to the specified address
    pub fn bind<P: AsRef<Path> + ?Sized>(&self, addr: &P) -> io::Result<()> {
        net::bind(&self.io, &try!(to_nix_addr(addr)))
    }

    pub fn try_clone(&self) -> io::Result<UnixSocket> {
        net::dup(&self.io)
            .map(From::from)
    }

    pub fn shutdown(&self, how : Shutdown) -> io::Result<usize> {
        try!(nix::shutdown(self.as_raw_fd(), how)
            .map_err(super::from_nix_error));
        Ok(0)
    }

    pub fn read_recv_fd(&mut self, buf: &mut [u8]) -> io::Result<(usize, Option<RawFd>)> {
        let iov = [nix::IoVec::from_mut_slice(buf)];
        let mut cmsgspace: nix::CmsgSpace<[RawFd; 1]> = nix::CmsgSpace::new();
        let msg = try!(nix::recvmsg(self.io.as_raw_fd(), &iov, Some(&mut cmsgspace), MsgFlags::empty())
                           .map_err(super::from_nix_error));
        let mut fd = None;
        for cmsg in msg.cmsgs() {
            if let nix::ControlMessage::ScmRights(fds) = cmsg {
                // statically, there is room for at most one fd
                if fds.len() == 1 {
                    fd = Some(fds[0]);
                    break;
                }
            }
        }
        Ok((msg.bytes, fd))
    }

    pub fn write_send_fd(&mut self, buf: &[u8], fd: RawFd) -> io::Result<usize> {
        let iov = [nix::IoVec::from_slice(buf)];
        let fds = [fd];
        let cmsg = nix::ControlMessage::ScmRights(&fds);
        nix::sendmsg(self.io.as_raw_fd(),&iov, &[cmsg], MsgFlags::empty(), None)
            .map_err(super::from_nix_error)
    }
}

impl Read for UnixSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.read(buf)
    }
}

impl Write for UnixSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl Evented for UnixSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        self.io.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &Poll, token: Token, interest: Ready, opts: PollOpt) -> io::Result<()> {
        self.io.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.io.deregister(poll)
    }
}


impl From<Io> for UnixSocket {
    fn from(io: Io) -> UnixSocket {
        UnixSocket { io: io }
    }
}

impl FromRawFd for UnixSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> UnixSocket {
        UnixSocket { io: Io::from_raw_fd(fd) }
    }
}

impl IntoRawFd for UnixSocket {
    fn into_raw_fd(self) -> RawFd {
        self.io.into_raw_fd()
    }
}

impl AsRawFd for UnixSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.io.as_raw_fd()
    }
}

fn to_nix_addr<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<nix::SockAddr> {
    nix::SockAddr::new_unix(path.as_ref())
        .map_err(super::from_nix_error)
}
