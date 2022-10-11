#![no_std]
#![feature(linkage)]

// pub mod framebuffer;
pub mod io;
// pub mod sync;
// pub mod thread;

// #[no_mangle]
// #[link_section = ".rodata.SceModuleInfo"]
// #[linkage = "weak"]
// pub static sceLibcHeapSize: usize = usize::MAX;

// #[no_mangle]
// #[link_section = ".rodata.SceModuleInfo"]
// #[linkage = "weak"]
// pub static sceLibcHeapExtendedAlloc: libc::SceBool = libc::SCE_TRUE;

extern "C" {
    #[link_name = "main"]
    pub fn c_main(argc: isize, argv: *const *const u8) -> isize;
}

#[macro_export]
macro_rules! module_start {
    () => {
        pub mod sce_module {
            #[no_mangle]
            pub unsafe extern "C" fn module_start(_argc: isize, _argv: *const u8) -> isize {
                const ARGS: &[*const u8] = &[::core::ptr::null_mut()];
                $crate::c_main(0, ARGS.as_ptr())
            }
        }
    };
}
