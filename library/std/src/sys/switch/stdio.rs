use crate::io::{self, IoSlice, IoSliceMut};

pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin(())
    }
}

#[allow(unused_variables)]
impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unimplemented!()
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        unimplemented!()
    }
}

extern "C" {
    pub fn skyline_tcp_send_raw(bytes: *const u8, usize: u64);
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout(())
    }
}

#[allow(unused_variables)]
impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            skyline_tcp_send_raw(buf.as_ptr(), buf.len() as u64);
        }

        Ok(buf.len())
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unimplemented!()
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr(())
    }
}

#[allow(unused_variables)]
impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            skyline_tcp_send_raw(buf.as_ptr(), buf.len() as u64);
        }
        Ok(buf.len())
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unimplemented!()
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0x10;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
