#![stable(feature = "rust1", since = "1.0.0")]

mod fd;
mod raw;

#[unstable(feature = "io_safety", issue = "87074")]
pub use fd::*;
// #[stable(feature = "rust1", since = "1.0.0")]
pub use raw::*;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
}