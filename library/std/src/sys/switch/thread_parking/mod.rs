//! Thread parking without `futex` using the `pthread` synchronization primitives.

use crate::cell::UnsafeCell;
use crate::marker::PhantomPinned;
use crate::pin::Pin;
use crate::ptr::addr_of_mut;
use crate::sync::atomic::AtomicUsize;
use crate::sync::atomic::Ordering::SeqCst;
use crate::time::Duration;

const EMPTY: usize = 0;
const PARKED: usize = 1;
const NOTIFIED: usize = 2;

unsafe fn lock(lock: *mut libc::pthread_mutex_t) {
    let r = libc::pthread_mutex_lock(lock);
    debug_assert_eq!(r, 0);
}

unsafe fn unlock(lock: *mut libc::pthread_mutex_t) {
    let r = libc::pthread_mutex_unlock(lock);
    debug_assert_eq!(r, 0);
}

unsafe fn notify_one(cond: *mut libc::pthread_cond_t) {
    let r = libc::pthread_cond_signal(cond);
    debug_assert_eq!(r, 0);
}

unsafe fn wait(cond: *mut libc::pthread_cond_t, lock: *mut libc::pthread_mutex_t) {
    let r = libc::pthread_cond_wait(cond, lock);
    debug_assert_eq!(r, 0);
}

unsafe fn wait_timeout(
    cond: *mut libc::pthread_cond_t,
    lock: *mut libc::pthread_mutex_t,
    dur: Duration,
) {
    
}

pub struct Parker {
    state: AtomicUsize,
    lock: UnsafeCell<libc::pthread_mutex_t>,
    cvar: UnsafeCell<libc::pthread_cond_t>,
    // The `pthread` primitives require a stable address, so make this struct `!Unpin`.
    _pinned: PhantomPinned,
}

impl Parker {
    /// Construct the UNIX parker in-place.
    ///
    /// # Safety
    /// The constructed parker must never be moved.
    pub unsafe fn new_in_place(parker: *mut Parker) {
        unimplemented!();
    }

    // This implementation doesn't require `unsafe`, but other implementations
    // may assume this is only called by the thread that owns the Parker.
    pub unsafe fn park(self: Pin<&Self>) {
        unimplemented!();
    }

    // This implementation doesn't require `unsafe`, but other implementations
    // may assume this is only called by the thread that owns the Parker. Use
    // `Pin` to guarantee a stable address for the mutex and condition variable.
    pub unsafe fn park_timeout(self: Pin<&Self>, dur: Duration) {
        unimplemented!();
    }

    pub fn unpark(self: Pin<&Self>) {
        unimplemented!();
    }
}

impl Drop for Parker {
    fn drop(&mut self) {
        unimplemented!();
    }
}

unsafe impl Sync for Parker {}
unsafe impl Send for Parker {}
