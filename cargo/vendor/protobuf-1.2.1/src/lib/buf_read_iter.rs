use std::cmp;
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::mem;

/// Dangerous implementation of `BufRead`.
///
/// Unsafe wrapper around BufRead which assumes that `BufRead` buf is
/// not moved when `BufRead` is moved.
///
/// This assumption is generally incorrect, however, in practice
/// `BufReadIter` is created either from `BufRead` reference (which
/// cannot  be moved, because it is locked by `CodedInputStream`) or from
/// `BufReader` which does not move its buffer (we know that from
/// inspecting rust standard library).
///
/// It is important for `CodedInputStream` performance that small reads
/// (e. g. 4 bytes reads) do not involve virtual calls or switches.
/// This is achievable with `BufReadIter`.
pub struct BufReadIter<R : BufRead> {
    buf_read: R,
    buf: &'static [u8],
    pos: usize, // within buf
}

impl<R : BufRead> BufReadIter<R> {
    pub fn new(buf_read: R) -> BufReadIter<R> {
        BufReadIter {
            buf_read: buf_read,
            buf: &[],
            pos: 0,
        }
    }

    #[inline]
    pub fn remaining(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    #[inline]
    pub fn eof(&mut self) -> io::Result<bool> {
        self.fill_buf()?;
        Ok(self.buf.is_empty())
    }

    #[inline]
    pub fn read_byte(&mut self) -> io::Result<u8> {
        self.fill_buf()?;
        if self.pos == self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
        }

        let r = self.buf[self.pos];
        self.pos += 1;
        Ok(r)
    }
}

impl<R : BufRead> Drop for BufReadIter<R> {
    fn drop(&mut self) {
        self.buf_read.consume(self.pos);
    }
}

impl<R : BufRead> Read for BufReadIter<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fill_buf()?;

        let rem = &self.buf[self.pos..];

        let len = cmp::min(rem.len(), buf.len());
        &mut buf[..len].copy_from_slice(&rem[..len]);
        self.pos += len;
        Ok((len))
    }
}

impl<R : BufRead> BufRead for BufReadIter<R> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos == self.buf.len() {
            self.buf_read.consume(self.pos);

            // Danger! `buf_read.buf` must not be moved!
            self.buf = unsafe { mem::transmute(self.buf_read.fill_buf()?) };
            self.pos = 0;
        }

        Ok(&self.buf[self.pos..])
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        assert!(amt < self.buf.len() - self.pos);
        self.pos += amt;
    }
}
