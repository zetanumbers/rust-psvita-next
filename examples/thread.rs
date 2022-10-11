#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::{fmt::Write, mem, ptr};

use psp2::io;

const ARGLEN: usize = 64;

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let _ = core::writeln!(io::File::stderr(), "{panic_info}");
    // core::intrinsics::abort();
    unsafe {
        libc::sceKernelExitProcess(libc::EXIT_FAILURE);
        loop {
            libc::sceKernelDelayThread(10);
        }
    }
}

// #[no_mangle]
// pub extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
//     unsafe { libc::abort() }
// }

unsafe fn spawn(
    entry: libc::SceKernelThreadEntry,
    arglen: libc::SceSize,
    argp: *mut libc::c_void,
) -> libc::SceUID {
    const NAME: &str = "\0";
    let thrd = libc::sceKernelCreateThread(
        NAME.as_ptr().cast(),
        entry,
        0x10000100,
        2 * 1024 * 1024,
        0,
        libc::SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT,
        ptr::null_mut(),
    );
    assert!(thrd > 0, "{thrd}");
    assert_eq!(0, libc::sceKernelStartThread(thrd, arglen, argp));
    thrd
}

unsafe fn thread_info(thread: libc::SceUID) -> libc::SceKernelThreadInfo {
    let mut info = mem::MaybeUninit::<libc::SceKernelThreadInfo>::uninit();
    *ptr::addr_of_mut!((*info.as_mut_ptr()).size) = mem::size_of_val(&info);
    assert_eq!(0, libc::sceKernelGetThreadInfo(thread, info.as_mut_ptr()));
    info.assume_init()
}

unsafe extern "C" fn somebody_loops(arglen: usize, argp: *mut libc::c_void) -> i32 {
    assert_eq!(arglen, ARGLEN);
    assert_eq!(
        &[0u64; 8],
        core::slice::from_raw_parts(argp.cast::<u64>(), 8)
    );
    libc::free(argp);
    loop {
        assert_eq!(0, libc::sceKernelDelayThread(10));
    }
}

#[no_mangle]
pub unsafe extern "C" fn module_start(_args: libc::c_uint, _argp: *mut libc::c_char) {
    assert_eq!(0, libc::sceKernelDelayThread(1_000_000));
    let t = spawn(somebody_loops, ARGLEN, libc::calloc(8, 8));
    panic!("{:#?}", thread_info(t));
}

#[link(name = "SceKernelThreadMgr_stub")]
#[link(name = "SceLibKernel_stub")]
#[link(name = "SceLibc_stub")]
extern "C" {}
