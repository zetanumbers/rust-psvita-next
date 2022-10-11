#![feature(linkage)]
#![no_std]
#![no_main]

use core::{fmt::Write, mem};

use psp2::io;

#[no_mangle]
pub unsafe extern "C" fn module_start(_args: libc::c_uint, _argp: *mut libc::c_char) {
    writeln!(io::File::stderr(), "Initial:").unwrap();
    print_malloc_stats();

    let p = libc::malloc(0x100000);
    assert!(!p.is_null());
    writeln!(io::File::stderr(), "Malloc:").unwrap();
    print_malloc_stats();

    libc::free(p);
    writeln!(io::File::stderr(), "Free:").unwrap();
    print_malloc_stats();

    libc::exit(libc::EXIT_SUCCESS);
}

fn print_malloc_stats() {
    let mut mmsize = unsafe { mem::zeroed() };
    let err = unsafe { malloc_stats(&mut mmsize) };
    assert_eq!(err, 0);
    writeln!(io::File::stderr(), "{mmsize:#?}\n").unwrap();
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let _ = core::writeln!(io::File::stderr(), "{panic_info}");
    unsafe { libc::abort() }
}

#[link(name = "SceLibc_stub")]
extern "C" {
    fn malloc_stats(mmsize: *mut malloc_managed_size) -> libc::c_int;
}

#[allow(nonstandard_style)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(C)]
struct malloc_managed_size {
    max_system_size: usize,
    current_system_size: usize,
    max_inuse_size: usize,
    current_inuse_size: usize,
}
