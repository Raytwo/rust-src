//! Raw Unix-like file descriptors.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::fs;
use crate::io;
use crate::os::raw;
//use crate::sys_common::{AsInner, IntoInner};

/// Raw file descriptors.
#[stable(feature = "rust1", since = "1.0.0")]
pub type RawFd = raw::c_int;

/// A trait to extract the raw file descriptor from an underlying object.
///
/// This is only available on unix and WASI platforms and must be imported in
/// order to call the method. Windows platforms have a corresponding
/// `AsRawHandle` and `AsRawSocket` set of traits.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawFd {
    /// Extracts the raw file descriptor.
    ///
    /// This function is typically used to **borrow** an owned file descriptor.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw file descriptor to the caller, and the file descriptor is only
    /// guaranteed to be valid while the original object has not yet been
    /// destroyed.
    ///
    /// However, borrowing is not strictly required. See [`AsFd::as_fd`]
    /// for an API which strictly borrows a file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(unix)]
    /// use std::os::unix::io::{AsRawFd, RawFd};
    /// #[cfg(target_os = "wasi")]
    /// use std::os::wasi::io::{AsRawFd, RawFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// // Note that `raw_fd` is only valid as long as `f` exists.
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.as_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_fd(&self) -> RawFd;
}

/// A trait to express the ability to construct an object from a raw file
/// descriptor.
#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawFd {
    /// Constructs a new instance of `Self` from the given raw file
    /// descriptor.
    ///
    /// This function is typically used to **consume ownership** of the
    /// specified file descriptor. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// [`From<OwnedFd>::from`] implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `fd` passed in must be a valid an open file descriptor.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(unix)]
    /// use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
    /// #[cfg(target_os = "wasi")]
    /// use std::os::wasi::io::{FromRawFd, IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// // SAFETY: no other functions should call `from_raw_fd`, so there
    /// // is only one owner for the file descriptor.
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let f = unsafe { File::from_raw_fd(raw_fd) };
    /// # Ok::<(), io::Error>(())
    /// ```
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

/// A trait to express the ability to consume an object and acquire ownership of
/// its raw file descriptor.
#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawFd {
    /// Consumes this object, returning the raw underlying file descriptor.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// file descriptor to the caller. When used in this way, callers are then the unique
    /// owners of the file descriptor and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// [`Into<OwnedFd>::into`] implementation for an API which strictly
    /// transfers ownership.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::fs::File;
    /// # use std::io;
    /// #[cfg(unix)]
    /// use std::os::unix::io::{IntoRawFd, RawFd};
    /// #[cfg(target_os = "wasi")]
    /// use std::os::wasi::io::{IntoRawFd, RawFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// #[cfg(any(unix, target_os = "wasi"))]
    /// let raw_fd: RawFd = f.into_raw_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_fd(self) -> RawFd;
}

#[stable(feature = "raw_fd_reflexive_traits", since = "1.48.0")]
impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}
#[stable(feature = "raw_fd_reflexive_traits", since = "1.48.0")]
impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}
#[stable(feature = "raw_fd_reflexive_traits", since = "1.48.0")]
impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> RawFd {
        fd
    }
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as a consumed argument or returned as an owned value, and it never
/// has the value `-1`.
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)]
#[rustc_nonnull_optimization_guaranteed]
#[stable(feature = "io_safety", since = "1.63.0")]
pub struct OwnedFd {
    pub fd: RawFd,
}


#[stable(feature = "io_safety", since = "1.63.0")]
impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// ownership. The resource must not require any cleanup other than `close`.
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        assert_ne!(fd, u32::MAX as RawFd);
        // SAFETY: we just asserted that the value is in the valid range and isn't `-1` (the only value bigger than `0xFF_FF_FF_FE` unsigned)
        unsafe { Self { fd } }
    }
}

//use crate::os::fd::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::sys_common::{self, AsInner, FromInner, IntoInner};
use crate::{net, sys};

macro_rules! impl_as_raw_fd {
    ($($t:ident)*) => {$(
        #[stable(feature = "rust1", since = "1.0.0")]
        impl AsRawFd for net::$t {
            #[inline]
            fn as_raw_fd(&self) -> RawFd {
                self.as_inner().socket().as_raw_fd()
            }
        }
    )*};
}
impl_as_raw_fd! { TcpStream TcpListener UdpSocket }

macro_rules! impl_from_raw_fd {
    ($($t:ident)*) => {$(
        #[stable(feature = "from_raw_os", since = "1.1.0")]
        impl FromRawFd for net::$t {
            #[inline]
            unsafe fn from_raw_fd(fd: RawFd) -> net::$t {
                unsafe {
                    let socket = sys::net::Socket::from_inner(OwnedFd::from_raw_fd(fd).fd);
                    net::$t::from_inner(sys_common::net::$t::from_inner(socket))
                }
            }
        }
    )*};
}
impl_from_raw_fd! { TcpStream TcpListener UdpSocket }

macro_rules! impl_into_raw_fd {
    ($($t:ident)*) => {$(
        #[stable(feature = "into_raw_os", since = "1.4.0")]
        impl IntoRawFd for net::$t {
            #[inline]
            fn into_raw_fd(self) -> RawFd {
                self.into_inner().into_socket().into_inner()//into_inner().into_raw_fd()
            }
        }
    )*};
}
impl_into_raw_fd! { TcpStream TcpListener UdpSocket }
