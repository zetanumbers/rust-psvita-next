#![no_std]
#![no_main]

use core::{cell::UnsafeCell, fmt::Write, mem, ptr};

use psp2::io;

struct State {
    mtx: UnsafeCell<libc::SceKernelLwMutexWork>,
    cond: UnsafeCell<libc::SceKernelLwCondWork>,
}

impl State {
    const fn new() -> Self {
        Self {
            mtx: UnsafeCell::new(libc::SceKernelLwMutexWork::ZEROED),
            cond: UnsafeCell::new(libc::SceKernelLwCondWork::ZEROED),
        }
    }

    unsafe fn init(&self) {
        const MTX_NAME: &str = "rust-psvita\0";
        let err = libc::sceKernelCreateLwMutex(
            self.mtx.get(),
            MTX_NAME.as_ptr().cast(),
            libc::SCE_KERNEL_MUTEX_ATTR_RECURSIVE,
            // 0,
            0,
            ptr::null_mut(),
        );
        assert_eq!(err, 0);

        const COND_NAME: &str = "rust-psvita\0";
        let err = libc::sceKernelCreateLwCond(
            self.cond.get(),
            COND_NAME.as_ptr().cast(),
            0,
            self.mtx.get(),
            ptr::null(),
        );
        assert_eq!(err, 0);
    }

    unsafe fn destroy(&self) {
        assert_eq!(0, libc::sceKernelDeleteLwCond(self.cond.get()));
        assert_eq!(0, libc::sceKernelDeleteLwMutex(self.mtx.get()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn module_start(_args: libc::c_uint, _argp: *mut libc::c_char) {
    let state = State::new();
    state.init();

    const THRD_NAME: &str = "rust-psvita\0";
    unsafe extern "C" fn entry(arglen: usize, state: *mut libc::c_void) -> libc::c_int {
        assert_eq!(arglen, mem::size_of::<State>());
        let state = &*state.cast::<State>();

        assert_eq!(
            0,
            libc::sceKernelLockLwMutex(state.mtx.get(), 1, ptr::null_mut())
        );
        assert_eq!(
            libc::SCE_KERNEL_ERROR_WAIT_TIMEOUT,
            libc::sceKernelWaitLwCond(state.cond.get(), ptr::null_mut())
        );
        assert_eq!(0, libc::sceKernelUnlockLwMutex(state.mtx.get(), 1));

        0
    }

    // assert_eq!(
    //     0,
    //     libc::sceKernelLockLwMutex(state.mtx.get(), 1, ptr::null_mut())
    // );

    // let err = entry(
    //     mem::size_of_val(&state),
    //     ptr::addr_of!(state) as *mut State as _,
    // );
    // assert_eq!(err, 0);

    let thrd = libc::sceKernelCreateThread(
        THRD_NAME.as_ptr().cast(),
        entry,
        0x10000100,
        0x10000,
        0,
        libc::SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT,
        ptr::null_mut(),
    );
    assert!(thrd > 0, "{thrd}");

    assert_eq!(
        0,
        libc::sceKernelStartThread(
            thrd,
            mem::size_of_val(&state),
            ptr::addr_of!(state) as *mut State as _
        )
    );

    // assert_eq!(0, libc::sceKernelDelayThread(250_000));

    // assert_eq!(0, libc::sceKernelTryLockLwMutex(state.mtx.get(), 1));

    // let mut mtx_info = libc::SceKernelMutexInfo {
    //     size: core::mem::size_of::<libc::SceKernelMutexInfo>(),
    //     ..core::mem::zeroed()
    // };
    // let err = libc::sceKernelGetMutexInfo(mtx, &mut mtx_info);
    // assert_eq!(err, 0);

    assert_eq!(
        0,
        libc::sceKernelWaitThreadEnd(thrd, &mut 0, ptr::null_mut())
    );

    // assert_eq!(0, libc::sceKernelUnlockLwMutex(state.mtx.get(), 1));

    // let err = libc::sceKernelGetMutexInfo(mtx, &mut mtx_info);
    // assert_eq!(err, 0);
    // core::writeln!(io::File::stderr(), "after wait mtx_info = {mtx_info:#?}").unwrap();

    assert_eq!(0, libc::sceKernelDeleteThread(thrd));

    state.destroy();

    libc::exit(libc::EXIT_SUCCESS);
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let _ = core::writeln!(io::File::stderr(), "{panic_info}");
    unsafe { libc::abort() }
}

#[link(name = "SceSysmem_stub")]
#[link(name = "SceLibc_stub")]
#[link(name = "SceDisplay_stub")]
#[link(name = "SceKernelThreadMgr_stub")]
#[link(name = "SceLibKernel_stub")]
#[link(name = "SceProcessmgr_stub")]
extern "C" {}
