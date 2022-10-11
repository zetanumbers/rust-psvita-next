use core::{cell::UnsafeCell, mem, num::NonZeroI32, ptr};

use crate::{
    io::{self, cvt_nz, cvt_p},
    sync::{self, RawMutex},
};

#[repr(u32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    NextFrame = libc::SCE_DISPLAY_SETBUF_NEXTFRAME,
    Immediate = libc::SCE_DISPLAY_SETBUF_IMMEDIATE,
}

#[derive(Clone, Copy, Debug)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl Resolution {
    pub const W960H544: Self = Resolution {
        width: 960,
        height: 544,
    };
    pub const W720H408: Self = Resolution {
        width: 720,
        height: 408,
    };
    pub const W640H368: Self = Resolution {
        width: 640,
        height: 368,
    };
    pub const W480H272: Self = Resolution {
        width: 480,
        height: 272,
    };

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }

    pub fn shape(self) -> [u32; 2] {
        [self.width, self.height]
    }
}

pub struct FrameBuffer {
    mtx: lock_api::Mutex,
    memblock: NonZeroI32,
    descriptor: libc::SceDisplayFrameBuf,
    base: ptr::NonNull<u8>,
    size: usize,
}

impl FrameBuffer {
    /// Zeroed framebuffer
    pub fn new(resolution: Resolution, mode: Mode) -> Self {
        const CDRAM_FB_PAGE_SIZE: usize = 256 * 1024;

        let pitch = 960;
        let [width, height] = resolution.shape();
        let size = (4 * (pitch * height) as usize + CDRAM_FB_PAGE_SIZE - 1) / CDRAM_FB_PAGE_SIZE
            * CDRAM_FB_PAGE_SIZE;
        let mut base = ptr::null_mut();

        let mtx = lock_api::Mutex::new().unwrap();
        let memblock = cvt_p(unsafe {
            libc::sceKernelAllocMemBlock(
                b"display\0".as_ptr().cast(),
                libc::SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW,
                size,
                ptr::null(),
            )
        })
        .unwrap();
        cvt_nz(unsafe { libc::sceKernelGetMemBlockBase(memblock.get(), &mut base) }).unwrap();
        debug_assert!(!base.is_null());

        let base = unsafe { ptr::NonNull::new_unchecked(base.cast::<u8>()) };
        unsafe {
            base.as_ptr().write_bytes(0, size);
        }

        let descriptor = libc::SceDisplayFrameBuf {
            size: mem::size_of::<libc::SceDisplayFrameBuf>(),
            base: base.as_ptr().cast(),
            pitch,
            pixelformat: libc::SCE_DISPLAY_PIXELFORMAT_A8B8G8R8,
            width,
            height,
        };
        cvt_nz(unsafe { libc::sceDisplaySetFrameBuf(&descriptor, mode as _) }).unwrap();
        cvt_nz(unsafe { libc::sceDisplayWaitSetFrameBuf() }).unwrap();

        FrameBuffer {
            base,
            mtx,
            memblock,
            descriptor,
            size,
        }
    }

    pub fn lock(&self) -> io::Result<()> {
        self.mtx.lock()
    }

    pub unsafe fn unlock(&self) -> io::Result<()> {
        self.mtx.unlock()
    }

    pub fn buffer(&self) -> &UnsafeCell<[u8]> {
        unsafe {
            &*(ptr::slice_from_raw_parts_mut(self.base.as_ptr(), self.size)
                as *const UnsafeCell<[u8]>)
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn resolution(&self) -> Resolution {
        let libc::SceDisplayFrameBuf { width, height, .. } = self.descriptor;
        Resolution { width, height }
    }

    pub fn width(&self) -> u32 {
        self.descriptor.width
    }

    pub fn height(&self) -> u32 {
        self.descriptor.height
    }

    pub fn pitch(&self) -> u32 {
        self.descriptor.pitch
    }
}

// TODO: FrameBuffer signleton stack?
// impl Drop for FrameBuffer {
//     fn drop(&mut self) {
//         self.descriptor.base = ptr::null_mut();
//         self.descriptor.pitch = 0;
//         self.descriptor.width = 0;
//         self.descriptor.height = 0;
//         // TODO: DRY
//         cvt_nz(unsafe { libc::sceDisplaySetFrameBuf(&self.descriptor, Mode::NextFrame as _) })
//             .expect("Failed to set empty frame buffer");
//         cvt_nz(unsafe { libc::sceDisplayWaitSetFrameBuf() })
//             .expect("Failed to set empty frame buffer");

//         cvt_nz(unsafe { libc::sceKernelFreeMemBlock(self.memblock.get()) }).unwrap();
//     }
// }
