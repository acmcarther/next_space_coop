use std::fmt;
use std::io::{self, Read, Write, ErrorKind};
use std::mem;
use std::net::{self, SocketAddr};
use std::sync::{Mutex, MutexGuard};

use miow;
use miow::iocp::CompletionStatus;
use miow::net::*;
use net2::{TcpBuilder, TcpStreamExt as Net2TcpExt};
use net::tcp::Shutdown;
use winapi::*;

use {Evented, Ready, Poll, PollOpt, Token};
use io::would_block;
use poll;
use sys::windows::from_raw_arc::FromRawArc;
use sys::windows::selector::{Overlapped, ReadyBinding};
use sys::windows::{wouldblock, Family};

pub struct TcpStream {
    /// Separately stored implementation to ensure that the `Drop`
    /// implementation on this type is only executed when it's actually dropped
    /// (many clones of this `imp` are made).
    imp: StreamImp,
    registration: Mutex<Option<poll::Registration>>,
}

pub struct TcpListener {
    imp: ListenerImp,
    registration: Mutex<Option<poll::Registration>>,
}

#[derive(Clone)]
struct StreamImp {
    /// A stable address and synchronized access for all internals. This serves
    /// to ensure that all `Overlapped` pointers are valid for a long period of
    /// time as well as allowing completion callbacks to have access to the
    /// internals without having ownership.
    ///
    /// Note that the reference count also allows us "loan out" copies to
    /// completion ports while I/O is running to guarantee that this stays alive
    /// until the I/O completes. You'll notice a number of calls to
    /// `mem::forget` below, and these only happen on successful scheduling of
    /// I/O and are paired with `overlapped2arc!` macro invocations in the
    /// completion callbacks (to have a decrement match the increment).
    inner: FromRawArc<StreamIo>,
}

#[derive(Clone)]
struct ListenerImp {
    inner: FromRawArc<ListenerIo>,
}

struct StreamIo {
    inner: Mutex<StreamInner>,
    read: Overlapped, // also used for connect
    write: Overlapped,
    socket: net::TcpStream,
}

struct ListenerIo {
    inner: Mutex<ListenerInner>,
    accept: Overlapped,
    family: Family,
    socket: net::TcpListener,
}

struct StreamInner {
    iocp: ReadyBinding,
    deferred_connect: Option<SocketAddr>,
    read: State<(), ()>,
    write: State<(Vec<u8>, usize), (Vec<u8>, usize)>,
}

struct ListenerInner {
    iocp: ReadyBinding,
    accept: State<net::TcpStream, (net::TcpStream, SocketAddr)>,
    accept_buf: AcceptAddrsBuf,
}

enum State<T, U> {
    Empty,              // no I/O operation in progress
    Pending(T),         // an I/O operation is in progress
    Ready(U),           // I/O has finished with this value
    Error(io::Error),   // there was an I/O error
}

impl TcpStream {
    fn new(socket: net::TcpStream,
           deferred_connect: Option<SocketAddr>) -> TcpStream {
        TcpStream {
            registration: Mutex::new(None),
            imp: StreamImp {
                inner: FromRawArc::new(StreamIo {
                    read: Overlapped::new(read_done),
                    write: Overlapped::new(write_done),
                    socket: socket,
                    inner: Mutex::new(StreamInner {
                        iocp: ReadyBinding::new(),
                        deferred_connect: deferred_connect,
                        read: State::Empty,
                        write: State::Empty,
                    }),
                }),
            },
        }
    }

    pub fn connect(socket: net::TcpStream, addr: &SocketAddr)
                   -> io::Result<TcpStream> {
        try!(socket.set_nonblocking(true));
        Ok(TcpStream::new(socket, Some(*addr)))
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.imp.inner.socket.peer_addr()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.imp.inner.socket.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<TcpStream> {
        self.imp.inner.socket.try_clone().map(|s| TcpStream::new(s, None))
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.imp.inner.socket.shutdown(how)
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.imp.inner.socket.set_nodelay(nodelay)
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.imp.inner.socket.nodelay()
    }

    pub fn set_keepalive_ms(&self, millis: Option<u32>) -> io::Result<()> {
        self.imp.inner.socket.set_keepalive_ms(millis)
    }

    pub fn keepalive_ms(&self) -> io::Result<Option<u32>> {
        self.imp.inner.socket.keepalive_ms()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.imp.inner.socket.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.imp.inner.socket.ttl()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.imp.inner.socket.take_error()
    }

    fn inner(&self) -> MutexGuard<StreamInner> {
        self.imp.inner()
    }

    fn post_register(&self, interest: Ready, me: &mut StreamInner) {
        if interest.is_readable() {
            self.imp.schedule_read(me);
        }

        // At least with epoll, if a socket is registered with an interest in
        // writing and it's immediately writable then a writable event is
        // generated immediately, so do so here.
        if interest.is_writable() {
            if let State::Empty = me.write {
                self.imp.add_readiness(me, Ready::writable());
            }
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut me = self.inner();

        match me.read {
            // Empty == we're not associated yet, and if we're pending then
            // these are both cases where we return "would block"
            State::Empty |
            State::Pending(()) => Err(wouldblock()),

            // If we got a delayed error as part of a `read_overlapped` below,
            // return that here. Also schedule another read in case it was
            // transient.
            State::Error(_) => {
                let e = match mem::replace(&mut me.read, State::Empty) {
                    State::Error(e) => e,
                    _ => panic!(),
                };
                self.imp.schedule_read(&mut me);
                Err(e)
            }

            // If we're ready for a read then some previous 0-byte read has
            // completed. In that case the OS's socket buffer has something for
            // us, so we just keep pulling out bytes while we can. Eventually
            // once we hit an error (which may include WouldBlock) we schedule
            // another 0-byte read which moves us to the `Pending` state.
            State::Ready(()) => {
                let res = (&self.imp.inner.socket).read(buf);
                if res.is_err() {
                    me.read = State::Empty;
                    self.imp.schedule_read(&mut me);
                }
                return res
            }
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut me = self.inner();
        let me = &mut *me;

        match me.write {
            State::Empty => {}
            _ => return Err(wouldblock())
        }

        if !me.iocp.registered() {
            return Err(wouldblock())
        }

        let mut intermediate = me.iocp.get_buffer(64 * 1024);
        let amt = try!(intermediate.write(buf));
        self.imp.schedule_write(intermediate, 0, me);
        Ok(amt)
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl StreamImp {
    fn inner(&self) -> MutexGuard<StreamInner> {
        self.inner.inner.lock().unwrap()
    }

    fn schedule_connect(&self, addr: &SocketAddr) -> io::Result<()> {
        unsafe {
            trace!("scheduling a connect");
            let overlapped = miow::Overlapped::from_raw(self.inner.read.as_mut_ptr());
            try!(self.inner.socket.connect_overlapped(addr, overlapped));
        }
        // see docs above on StreamImp.inner for rationale on forget
        mem::forget(self.clone());
        Ok(())
    }

    /// Schedule a read to happen on this socket, enqueuing us to receive a
    /// notification when a read is ready.
    ///
    /// Note that this does *not* work with a buffer. When reading a TCP stream
    /// we actually read into a 0-byte buffer so Windows will send us a
    /// notification when the socket is otherwise ready for reading. This allows
    /// us to avoid buffer allocations for in-flight reads.
    fn schedule_read(&self, me: &mut StreamInner) {
        match me.read {
            State::Empty => {}
            State::Ready(_) | State::Error(_) => {
                self.add_readiness(me, Ready::readable());
                return;
            }
            _ => return,
        }

        me.iocp.set_readiness(me.iocp.readiness() & !Ready::readable());

        trace!("scheduling a read");
        let res = unsafe {
            let overlapped = miow::Overlapped::from_raw(self.inner.read.as_mut_ptr());
            self.inner.socket.read_overlapped(&mut [], overlapped)
        };
        match res {
            // TODO: investigate better handling `Ok(true)`
            //
            // Note that `Ok(true)` means that this completed immediately and
            // our socket is readable. This typically means that the caller of
            // this function (likely `read` above) can try again as an
            // optimization and return bytes quickly.
            //
            // Unfortunately, though, although the read completed immediately
            // there's still an IOCP completion packet enqueued that we're going
            // to receive. As an ease of implementation for now we just let the
            // completion packet drive the read completion.
            //
            // Apparently you can configure this behavior with
            // SetFileCompletionNotificationModes to indicate that `Ok(true)`
            // does **not** enqueue a completion packet. We should test this out
            // and see if it works for us.
            //
            // Note that apparently libuv has scary code to work around bugs in
            // `WSARecv` for UDP sockets apparently for handles which have had
            // the `SetFileCompletionNotificationModes` function called on them,
            // worth looking into!
            Ok(_) => {
                // see docs above on StreamImp.inner for rationale on forget
                me.read = State::Pending(());
                mem::forget(self.clone());
            }
            Err(e) => {
                // Like above, be sure to indicate that hup has happened
                // whenever we get `ECONNRESET`
                let mut set = Ready::readable();
                if e.raw_os_error() == Some(WSAECONNRESET as i32) {
                    trace!("tcp stream at hup: econnreset");
                    set = set | Ready::hup();
                }
                me.read = State::Error(e);
                self.add_readiness(me, set);
            }
        }
    }

    /// Similar to `schedule_read`, except that this issues, well, writes.
    ///
    /// This function will continually attempt to write the entire contents of
    /// the buffer `buf` until they have all been written. The `pos` argument is
    /// the current offset within the buffer up to which the contents have
    /// already been written.
    ///
    /// A new writable event (e.g. allowing another write) will only happen once
    /// the buffer has been written completely (or hit an error).
    fn schedule_write(&self,
                      buf: Vec<u8>,
                      pos: usize,
                      me: &mut StreamInner) {

        // About to write, clear any pending level triggered events
        me.iocp.set_readiness(me.iocp.readiness() & !Ready::writable());

        trace!("scheduling a write");
        let err = unsafe {
            let overlapped = miow::Overlapped::from_raw(self.inner.write.as_mut_ptr());
            self.inner.socket.write_overlapped(&buf[pos..], overlapped)
        };
        match err {
            Ok(_) => {
                // see docs above on StreamImp.inner for rationale on forget
                me.write = State::Pending((buf, pos));
                mem::forget(self.clone());
            }
            Err(e) => {
                me.write = State::Error(e);
                self.add_readiness(me, Ready::writable());
                me.iocp.put_buffer(buf);
            }
        }
    }

    /// Pushes an event for this socket onto the selector its registered for.
    ///
    /// When an event is generated on this socket, if it happened after the
    /// socket was closed then we don't want to actually push the event onto our
    /// selector as otherwise it's just a spurious notification.
    fn add_readiness(&self, me: &mut StreamInner, set: Ready) {
        me.iocp.set_readiness(set | me.iocp.readiness());
    }
}

fn read_done(status: &OVERLAPPED_ENTRY) {
    let status = CompletionStatus::from_entry(status);
    let me2 = StreamImp {
        inner: unsafe { overlapped2arc!(status.overlapped(), StreamIo, read) },
    };

    let mut me = me2.inner();
    match mem::replace(&mut me.read, State::Empty) {
        State::Pending(()) => {
            trace!("finished a read: {}", status.bytes_transferred());
            assert_eq!(status.bytes_transferred(), 0);
            me.read = State::Ready(());
            return me2.add_readiness(&mut me, Ready::readable())
        }
        s => me.read = s,
    }

    // If a read didn't complete, then the connect must have just finished.
    trace!("finished a connect");

    match me2.inner.socket.connect_complete() {
        Ok(()) => {
            me2.add_readiness(&mut me, Ready::writable());
            me2.schedule_read(&mut me);
        }
        Err(e) => {
            me2.add_readiness(&mut me, Ready::readable());
            me.read = State::Error(e);
        }
    }
}

fn write_done(status: &OVERLAPPED_ENTRY) {
    let status = CompletionStatus::from_entry(status);
    trace!("finished a write {}", status.bytes_transferred());
    let me2 = StreamImp {
        inner: unsafe { overlapped2arc!(status.overlapped(), StreamIo, write) },
    };
    let mut me = me2.inner();
    let (buf, pos) = match mem::replace(&mut me.write, State::Empty) {
        State::Pending(pair) => pair,
        _ => unreachable!(),
    };
    let new_pos = pos + (status.bytes_transferred() as usize);
    if new_pos == buf.len() {
        me2.add_readiness(&mut me, Ready::writable());
    } else {
        me2.schedule_write(buf, new_pos, &mut me);
    }
}

impl Evented for TcpStream {
    fn register(&self, poll: &Poll, token: Token,
                interest: Ready, opts: PollOpt) -> io::Result<()> {
        let mut me = self.inner();
        try!(me.iocp.register_socket(&self.imp.inner.socket, poll, token,
                                     interest, opts, &self.registration));

        // If we were connected before being registered process that request
        // here and go along our merry ways. Note that the callback for a
        // successful connect will worry about generating writable/readable
        // events and scheduling a new read.
        if let Some(addr) = me.deferred_connect.take() {
            return self.imp.schedule_connect(&addr).map(|_| ())
        }
        self.post_register(interest, &mut me);
        Ok(())
    }

    fn reregister(&self, poll: &Poll, token: Token,
                  interest: Ready, opts: PollOpt) -> io::Result<()> {
        let mut me = self.inner();
        try!(me.iocp.reregister_socket(&self.imp.inner.socket, poll, token,
                                       interest, opts, &self.registration));
        self.post_register(interest, &mut me);
        Ok(())
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.inner().iocp.deregister(&self.imp.inner.socket,
                                     poll, &self.registration)
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "TcpStream { ... }".fmt(f)
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        // If we're still internally reading, we're no longer interested. Note
        // though that we don't cancel any writes which may have been issued to
        // preserve the same semantics as Unix.
        //
        // Note that "Empty" here may mean that a connect is pending, so we
        // cancel even if that happens as well.
        unsafe {
            match self.inner().read {
                State::Pending(_) | State::Empty => {
                    trace!("cancelling active TCP read");
                    drop(super::cancel(&self.imp.inner.socket,
                                       &self.imp.inner.read));
                }
                State::Ready(_) | State::Error(_) => {}
            }
        }
    }
}

impl TcpListener {
    pub fn new(socket: net::TcpListener, addr: &SocketAddr)
               -> io::Result<TcpListener> {
        Ok(TcpListener::new_family(socket, match *addr {
            SocketAddr::V4(..) => Family::V4,
            SocketAddr::V6(..) => Family::V6,
        }))
    }

    fn new_family(socket: net::TcpListener, family: Family) -> TcpListener {
        TcpListener {
            registration: Mutex::new(None),
            imp: ListenerImp {
                inner: FromRawArc::new(ListenerIo {
                    accept: Overlapped::new(accept_done),
                    family: family,
                    socket: socket,
                    inner: Mutex::new(ListenerInner {
                        iocp: ReadyBinding::new(),
                        accept: State::Empty,
                        accept_buf: AcceptAddrsBuf::new(),
                    }),
                }),
            },
        }
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut me = self.inner();

        let ret = match mem::replace(&mut me.accept, State::Empty) {
            State::Empty => return Err(would_block()),
            State::Pending(t) => {
                me.accept = State::Pending(t);
                return Err(would_block());
            }
            State::Ready((s, a)) => {
                try!(s.set_nonblocking(true));
                Ok((TcpStream::new(s, None), a))
            }
            State::Error(e) => Err(e),
        };

        self.imp.schedule_accept(&mut me);

        return ret
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.imp.inner.socket.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<TcpListener> {
        self.imp.inner.socket.try_clone().map(|s| {
            TcpListener::new_family(s, self.imp.inner.family)
        })
    }

    pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
        self.imp.inner.socket.set_only_v6(only_v6)
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.imp.inner.socket.only_v6()
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.imp.inner.socket.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.imp.inner.socket.ttl()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.imp.inner.socket.take_error()
    }

    fn inner(&self) -> MutexGuard<ListenerInner> {
        self.imp.inner()
    }
}

impl ListenerImp {
    fn inner(&self) -> MutexGuard<ListenerInner> {
        self.inner.inner.lock().unwrap()
    }

    fn schedule_accept(&self, me: &mut ListenerInner) {
        match me.accept {
            State::Empty => {}
            _ => return
        }

        me.iocp.set_readiness(me.iocp.readiness() & !Ready::readable());

        let res = match self.inner.family {
            Family::V4 => TcpBuilder::new_v4(),
            Family::V6 => TcpBuilder::new_v6(),
        }.and_then(|builder| unsafe {
            trace!("scheduling an accept");
            let overlapped = miow::Overlapped::from_raw(self.inner.accept.as_mut_ptr());
            self.inner.socket.accept_overlapped(&builder, &mut me.accept_buf,
                                                overlapped)
        });
        match res {
            Ok((socket, _)) => {
                // see docs above on StreamImp.inner for rationale on forget
                me.accept = State::Pending(socket);
                mem::forget(self.clone());
            }
            Err(e) => {
                me.accept = State::Error(e);
                self.add_readiness(me, Ready::readable());
            }
        }
    }

    // See comments in StreamImp::push
    fn add_readiness(&self, me: &mut ListenerInner, set: Ready) {
        me.iocp.set_readiness(set | me.iocp.readiness());
    }
}

fn accept_done(status: &OVERLAPPED_ENTRY) {
    let status = CompletionStatus::from_entry(status);
    let me2 = ListenerImp {
        inner: unsafe { overlapped2arc!(status.overlapped(), ListenerIo, accept) },
    };

    let mut me = me2.inner();
    let socket = match mem::replace(&mut me.accept, State::Empty) {
        State::Pending(s) => s,
        _ => unreachable!(),
    };
    trace!("finished an accept");
    let result = me2.inner.socket.accept_complete(&socket).and_then(|()| {
        me.accept_buf.parse(&me2.inner.socket)
    }).and_then(|buf| {
        buf.remote().ok_or_else(|| {
            io::Error::new(ErrorKind::Other, "could not obtain remote address")
        })
    });
    me.accept = match result {
        Ok(remote_addr) => State::Ready((socket, remote_addr)),
        Err(e) => State::Error(e),
    };
    me2.add_readiness(&mut me, Ready::readable());
}

impl Evented for TcpListener {
    fn register(&self, poll: &Poll, token: Token,
                interest: Ready, opts: PollOpt) -> io::Result<()> {
        let mut me = self.inner();
        try!(me.iocp.register_socket(&self.imp.inner.socket, poll, token,
                                     interest, opts, &self.registration));
        self.imp.schedule_accept(&mut me);
        Ok(())
    }

    fn reregister(&self, poll: &Poll, token: Token,
                  interest: Ready, opts: PollOpt) -> io::Result<()> {
        let mut me = self.inner();
        try!(me.iocp.reregister_socket(&self.imp.inner.socket, poll, token,
                                       interest, opts, &self.registration));
        self.imp.schedule_accept(&mut me);
        Ok(())
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.inner().iocp.deregister(&self.imp.inner.socket,
                                     poll, &self.registration)
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "TcpListener { ... }".fmt(f)
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        // If we're still internally reading, we're no longer interested.
        unsafe {
            match self.inner().accept {
                State::Pending(_) => {
                    trace!("cancelling active TCP accept");
                    drop(super::cancel(&self.imp.inner.socket,
                                       &self.imp.inner.accept));
                }
                State::Empty |
                State::Ready(_) |
                State::Error(_) => {}
            }
        }
    }
}