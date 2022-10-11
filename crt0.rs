#![no_std]

extern "C" {
    #[link_name = "main"]
    pub fn c_main(argc: isize, argv: *const *const u8) -> isize;
}

#[no_mangle]
pub unsafe extern "C" fn module_start(_argc: isize, _argp: *const i8) -> isize {
    const ARGS: &[*const u8] = &[core::ptr::null()];
    c_main(0, ARGS.as_ptr())
}
