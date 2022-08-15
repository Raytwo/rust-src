use crate::cell::UnsafeCell;
use crate::sys_common::lazy_box::{LazyBox, LazyInit};

pub struct RwLock {
    mode: UnsafeCell<isize>,
}

pub(crate) type MovableRwLock = LazyBox<RwLock>;

unsafe impl Send for RwLock {}
unsafe impl Sync for RwLock {} // no threads on wasm

impl LazyInit for RwLock {
    fn init() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl RwLock {
    pub const fn new() -> RwLock {
        RwLock { mode: UnsafeCell::new(0) }
    }

    #[inline]
    pub unsafe fn read(&self) {
        let mode = self.mode.get();
        if *mode >= 0 {
            *mode += 1;
        } else {
            rtabort!("rwlock locked for writing");
        }
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        let mode = self.mode.get();
        if *mode >= 0 {
            *mode += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    pub unsafe fn write(&self) {
        let mode = self.mode.get();
        if *mode == 0 {
            *mode = -1;
        } else {
            rtabort!("rwlock locked for reading")
        }
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        let mode = self.mode.get();
        if *mode == 0 {
            *mode = -1;
            true
        } else {
            false
        }
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        *self.mode.get() -= 1;
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        *self.mode.get() += 1;
    }

    #[inline]
    pub unsafe fn destroy(&self) {}
}

impl Drop for RwLock {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.destroy() };
    }
}
