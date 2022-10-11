#![no_std]
#![no_main]

use core::{fmt::Write, mem, ptr};

use psp2::io;

#[no_mangle]
pub unsafe extern "C" fn module_start(_args: libc::c_uint, _argp: *mut libc::c_char) {
    let mut mtx = ptr::null_mut();

    assert_eq!(
        libc::thrd_success,
        libc::mtx_init(&mut mtx, libc::mtx_plain)
    );

    assert_eq!(libc::thrd_success, libc::mtx_lock(&mut mtx));
    assert_eq!(libc::thrd_success, libc::mtx_lock(&mut mtx));

    // let err = libc::puts(strerror(libc::));
    // assert_eq!(err, 0);

    // let mut mtx = libc::SceKernelLwMutexWork::ZEROED;

    // const MTX_NAME: &[u8] = b"rust-psvita\0";
    // let err = libc::sceKernelCreateLwMutex(
    //     &mut mtx,
    //     MTX_NAME.as_ptr().cast(),
    //     // libc::SCE_KERNEL_MUTEX_ATTR_RECURSIVE,
    //     0,
    //     0,
    //     ptr::null_mut(),
    // );
    // assert_eq!(err, 0);

    // const THRD_NAME: &[u8] = b"rust-psvita\0";
    // unsafe extern "C" fn entry(arglen: usize, mtx: *mut libc::c_void) -> libc::c_int {
    //     assert_eq!(arglen, mem::size_of::<libc::SceKernelLwMutexWork>());
    //     let mtx = mtx.cast();
    //     let err = libc::sceKernelTryLockLwMutex(mtx, 1);
    //     assert_eq!(err, 0);
    //     // let err = libc::sceKernelDelayThread(10_000);
    //     // assert_eq!(err, 0);

    //     0
    // }

    // let err = libc::sceKernelLockLwMutex(&mut mtx, 1, ptr::null_mut());
    // assert_eq!(err, 0);

    // let err = libc::sceKernelLockLwMutex(&mut mtx, 1, ptr::null_mut());
    // assert_eq!(err, 0);

    // let err = entry(mem::size_of_val(&mtx), ptr::addr_of_mut!(mtx).cast());
    // assert_eq!(err, 0);

    // let thrd = libc::sceKernelCreateThread(
    //     THRD_NAME.as_ptr().cast(),
    //     entry,
    //     0x10000100,
    //     0x10000,
    //     0,
    //     libc::SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT,
    //     ptr::null_mut(),
    // );
    // assert!(thrd > 0, "{thrd}");

    // let err =
    //     libc::sceKernelStartThread(thrd, mem::size_of_val(&mtx), ptr::addr_of_mut!(mtx).cast());
    // assert_eq!(err, 0);

    // let err = libc::sceKernelDelayThread(1_000);
    // assert_eq!(err, 0);

    // let mut mtx_info = libc::SceKernelMutexInfo {
    //     size: core::mem::size_of::<libc::SceKernelMutexInfo>(),
    //     ..core::mem::zeroed()
    // };
    // let err = libc::sceKernelGetMutexInfo(mtx, &mut mtx_info);
    // assert_eq!(err, 0);
    // core::writeln!(io::File::stderr(), "before wait mtx_info = {mtx_info:#?}").unwrap();

    // let mut stat = 0;
    // let err = libc::sceKernelWaitThreadEnd(thrd, &mut stat, ptr::null_mut());
    // assert_eq!(err, 0);

    // let err = libc::sceKernelUnlockLwMutex(&mut mtx, 1);
    // assert_eq!(err, 0);

    // let err = libc::sceKernelDeleteLwMutex(&mut mtx);
    // assert_eq!(err, 0);

    // let err = libc::sceKernelGetMutexInfo(mtx, &mut mtx_info);
    // assert_eq!(err, 0);
    // core::writeln!(io::File::stderr(), "after wait mtx_info = {mtx_info:#?}").unwrap();

    // let err = libc::sceKernelDeleteThread(thrd);
    // assert_eq!(err, 0);

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
extern "C" {
    fn strerror(errnum: libc::c_int) -> *mut libc::c_char;
}
