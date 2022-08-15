//! Owned and borrowed Unix-like file descriptors.

#![stable(feature = "io_safety", since = "1.63.0")]
#![deny(unsafe_op_in_unsafe_fn)]

// `RawFd`, `AsRawFd`, etc.
#[cfg(not(target_os = "switch"))]
pub mod raw;

// `OwnedFd`, `AsFd`, etc.
#[cfg(not(target_os = "switch"))]
pub mod owned;

// Implementations for `AsRawFd` etc. for network types.
#[cfg(not(target_os = "switch"))]
mod net;

#[cfg(test)]
mod tests;
