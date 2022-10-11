use core::{num::NonZeroI32, ptr, sync::atomic};

use crate::io::{self, cvt_nz, cvt_p};

pub use core::sync::atomic::AtomicI32 as AtomicUID;

pub struct RawMutex {
    uid: AtomicUID,
}

impl RawMutex {
    pub const fn new() -> Self {
        RawMutex {
            uid: AtomicUID::new(0),
        }
    }

    pub fn lock_fallible(&self) -> io::Result<()> {
        let uid = loop {
            let state = self.uid.compare_exchange_weak(
                0,
                -1,
                atomic::Ordering::Acquire,
                atomic::Ordering::Relaxed,
            );
            match state {
                Err(uid) if uid > 0 => break uid,
                Err(-1..=0) => crate::thread::yield_now(),
                Err(-2) => {
                    // let the initial thread fail first
                    crate::thread::yield_now();
                    unreachable!(
                        "`sceKernelCreateMutex` returned a value in `-2..=0` on another thread"
                    )
                }
                Err(error) if error < 0 => {
                    return Err(io::Error(unsafe { NonZeroI32::new_unchecked(error) }))
                }

                Ok(0) => {
                    let mtx = unsafe {
                        libc::sceKernelCreateMutex(
                            b"rust psp2::sync::RawMutex\0".as_ptr().cast(),
                            <_>::default(),
                            0,
                            ptr::null_mut(),
                        )
                    };
                    if let -2..=0 = mtx {
                        self.uid.store(-2, atomic::Ordering::Release);
                        unreachable!("`sceKernelCreateMutex` returned {mtx}");
                    }
                    self.uid.store(mtx, atomic::Ordering::Release);
                    break mtx;
                }
                Ok(_) => unreachable!(),
            }
        };
        if uid < 0 {
            return Err(io::Error::from_raw_os_error(uid));
        }

        cvt_nz(unsafe { libc::sceKernelLockMutex(self.uid.get(), 1, ptr::null_mut()) })
    }

    pub unsafe fn unlock_fallible(&self) -> io::Result<()> {
        cvt_nz(libc::sceKernelUnlockMutex(self.uid.get(), 1))
    }
}
