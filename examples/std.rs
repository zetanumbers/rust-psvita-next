psp2::module_start!();

fn main() {
    // println!("Hello World!")
}

#[link(name = "SceLibc_stub")]
#[link(name = "SceLibKernel_stub")]
#[link(name = "SceKernelThreadMgr_stub")]
extern "C" {}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
    unsafe { libc::abort() }
}

#[no_mangle]
pub extern "C" fn __aeabi_read_tp() -> *mut core::ffi::c_void {
    let ptr;
    unsafe {
        core::arch::asm!("mrc p15,0,{0},c13,c0,3", out(reg) ptr);
    }
    ptr
}