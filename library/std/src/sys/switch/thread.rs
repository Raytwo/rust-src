use crate::cmp;
use crate::ffi::CStr;
use crate::io;
use crate::mem;
use crate::ptr;
use crate::sys::os;
use crate::time::Duration;
use crate::num::NonZeroUsize;
use crate::sys::unsupported;

use nnsdk::{
    TimeSpan,
    os::{
        SleepThread,
        ThreadType,
        SetThreadName,
        GetCurrentThread,
    },
};

#[cfg(not(target_os = "l4re"))]
pub const DEFAULT_MIN_STACK_SIZE: usize = 2 * 1024 * 1024;
#[cfg(target_os = "l4re")]
pub const DEFAULT_MIN_STACK_SIZE: usize = 1024 * 1024;

pub struct Thread {
    inner: *mut ThreadType,
}

// Some platforms may have pthread_t as a pointer in which case we still want
// a thread to be Send/Sync
unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        let args = Box::into_raw(box p);
        let mut inner = Box::leak(Box::new(ThreadType::new())) as *mut _;

        let stack_size = cmp::max(stack, 0x1000);

        // TODO: Round up stack size

        // match pthread_attr_setstacksize(&mut attr, stack_size) {
        //     0 => {}
        //     n => {
        //         assert_eq!(n, libc::EINVAL);
        //         // EINVAL means |stack_size| is either too small or not a
        //         // multiple of the system page size.  Because it's definitely
        //         // >= PTHREAD_STACK_MIN, it must be an alignment issue.
        //         // Round up to the nearest page and try again.
        //         let page_size = os::page_size();
        //         let stack_size =
        //             (stack_size + page_size - 1) & (-(page_size as isize - 1) as usize - 1);
        //         assert_eq!(libc::pthread_attr_setstacksize(&mut attr, stack_size), 0);
        //     }
        // };

        let stack = crate::alloc::alloc(crate::alloc::Layout::from_size_align(stack_size, 0x1000).unwrap());
        // let stack = Box::leak(Box::new([0u8;stack_size]));

        let ret = unsafe { nnsdk::os::CreateThread(inner, thread_main, args as *mut _, stack as _, stack_size, 16) };
        println!("created");

        return if ret != 0 {
            // The thread failed to start and as a result p was not consumed. Therefore, it is
            // safe to reconstruct the box so that it gets deallocated.
            drop(Box::from_raw(args));
            unsafe { libc::free(inner as _) };

            Err(io::Error::from_raw_os_error(ret as _))
        } else {
            // The thread was successfully created, start it
            unsafe { nnsdk::os::StartThread(inner) };
            println!("started");

            Ok(Thread { inner })
        };

        extern "C" fn thread_main(main: *mut libc::c_void) {
            unsafe {
                Box::from_raw(main as *mut Box<dyn FnOnce()>)();
                println!("executed");

            }
        }
    }

    pub fn yield_now() {
        unsafe { nnsdk::os::YieldThread() }
    }

    pub fn set_name(name: &CStr) {
        unsafe {
            SetThreadName(GetCurrentThread(), name.as_ptr() as _)
        }
    }

    pub fn sleep(dur: Duration) {
        let time_span = TimeSpan::nano(dur.as_nanos() as u64);

        unsafe {
            SleepThread(time_span);
        }
    }

    pub fn join(mut self) {
        unsafe {
            unsafe {
                nnsdk::os::WaitThread(self.inner);
                println!("joined");
            }
        }
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    unsupported()
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { nnsdk::os::DestroyThread(self.inner) }
        unsafe { libc::free(self.inner as _) };
        println!("destroyed");
    }
}

#[cfg_attr(test, allow(dead_code))]
pub mod guard {
    use crate::ops::Range;
    pub type Guard = Range<usize>;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}

pub fn available_concurrency() -> io::Result<NonZeroUsize> {
    unsupported()
}