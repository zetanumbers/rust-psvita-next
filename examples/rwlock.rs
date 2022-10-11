#![no_std]
#![no_main]

use core::{fmt::Write, mem, ptr};

use psp2::io;

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let _ = core::writeln!(io::File::stderr(), "{panic_info}");
    unsafe {
        libc::sceKernelExitProcess(libc::EXIT_FAILURE);
        loop {
            libc::sceKernelDelayThread(10);
        }
    }
}

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
        0x10,
        0,
        libc::SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT,
        ptr::null_mut(),
    );
    assert!(thrd > 0, "{thrd}");
    assert_eq!(0, libc::sceKernelStartThread(thrd, arglen, argp));
    thrd
}

static mut RWLOCK: libc::SceUID = 0;

unsafe extern "C" fn read_entry(arglen: libc::SceSize, argp: *mut libc::c_void) -> libc::c_int {
    assert_eq!(arglen, mem::size_of::<libc::SceUID>());
    let rwlock = *argp.cast::<libc::SceUID>();

    assert_eq!(0, libc::sceKernelLockReadRWLock(rwlock, ptr::null_mut()));
    loop {
        assert_eq!(0, libc::sceKernelDelayThread(10));
    }
}

unsafe extern "C" fn write_entry(arglen: libc::SceSize, argp: *mut libc::c_void) -> libc::c_int {
    assert_eq!(arglen, mem::size_of::<libc::SceUID>());
    let rwlock = *argp.cast::<libc::SceUID>();

    assert_eq!(0, libc::sceKernelLockWriteRWLock(rwlock, ptr::null_mut()));
    loop {
        assert_eq!(0, libc::sceKernelDelayThread(10));
    }
}

#[no_mangle]
pub unsafe extern "C" fn module_start(_args: libc::c_uint, _argp: *mut libc::c_char) {
    const NAME: &str = "\0";
    RWLOCK = libc::sceKernelCreateRWLock(
        NAME.as_ptr().cast(),
        libc::SCE_KERNEL_ATTR_THREAD_PRIO,
        ptr::null(),
    );
    assert!(RWLOCK > 0, "{RWLOCK}");

    // assert_eq!(0, libc::sceKernelLockWriteRWLock(RWLOCK, ptr::null_mut()));

    spawn(
        write_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );
    spawn(
        write_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );
    spawn(
        write_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );
    spawn(
        read_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );
    spawn(
        read_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );
    spawn(
        read_entry,
        mem::size_of::<libc::SceUID>(),
        ptr::addr_of_mut!(RWLOCK).cast(),
    );

    assert_eq!(0, libc::sceKernelDelayThread(1000000));

    panic!("done!");
    // libc::sceKernelExitProcess(libc::EXIT_SUCCESS);
}

#[link(name = "SceKernelThreadMgr_stub")]
#[link(name = "SceLibKernel_stub")]
#[link(name = "SceLibc_stub")]
extern "C" {}
