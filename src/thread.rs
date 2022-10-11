use crate::io::cvt_nz;

#[track_caller]
pub fn sleep(dur: core::time::Duration) {
    let us = dur.as_micros() + (dur.as_nanos() % 1000 + 999) / 1000;
    // TODO: Sequence these calls
    cvt_nz(unsafe { libc::sceKernelDelayThread(us.try_into().expect("Duration is too large")) })
        .expect("IO error");
}

pub fn yield_now() {
    unsafe { libc::thrd_yield() }
}
