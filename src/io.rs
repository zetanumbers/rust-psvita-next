use core::{
    num::{NonZeroI32, NonZeroU32},
    ptr,
};

// TODO: Drop impl?
pub struct File {
    inner: *mut libc::FILE,
}

impl File {
    pub unsafe fn new(inner: *mut libc::FILE) -> Self {
        File { inner }
    }

    pub fn stdout() -> Self {
        unsafe { File::new(ptr::addr_of_mut!(libc::stdout)) }
    }

    pub fn stderr() -> Self {
        unsafe { File::new(ptr::addr_of_mut!(libc::stderr)) }
    }

    pub fn stdin() -> Self {
        unsafe { File::new(ptr::addr_of_mut!(libc::stdin)) }
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        unsafe { libc::fwrite(buf.as_ptr().cast(), 1, buf.len(), self.inner) }
    }

    pub fn flush(&mut self) -> Result<()> {
        unsafe { cvt_nz(libc::fflush(self.inner)) }
    }
}

impl core::fmt::Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut buf = s.as_bytes();
        while !buf.is_empty() {
            let written = self.write(buf);
            if written == 0 {
                return Err(core::fmt::Error);
            }
            buf = &buf.get(written..).ok_or(core::fmt::Error)?;
        }
        self.flush().map_err(|_| core::fmt::Error)?;
        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;

// TODO: figure out if this error is applicable to everything
#[derive(Clone, Copy, Debug)]
pub struct Error(pub NonZeroI32);

// impl From<Error> for rand::Error {
//     fn from(e: Error) -> Self {
//         unsafe { NonZeroU32::new_unchecked(e.0.get() as u32).into() }
//     }
// }

impl Error {
    pub unsafe fn from_raw_os_error_unchecked(code: i32) -> Error {
        Error(NonZeroI32::new_unchecked(code))
    }

    pub fn from_raw_os_error(code: i32) -> Error {
        Error(NonZeroI32::new(code).unwrap())
    }

    fn new(error: libc::c_int) -> Self {
        Error::try_new(error).unwrap_or_else(|e| {
            panic!("Error codes are always negative, but got positive anyway: {e}")
        })
    }

    fn try_new(error: libc::c_int) -> core::result::Result<Self, NonZeroI32> {
        let error = NonZeroI32::new(error)
            .expect("Zero is assumed to be reserved to \"empty\" values, but got it anyway");

        if error.get() < 0 {
            Ok(Error(error))
        } else {
            Err(error)
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn cvt_p(ret: libc::c_int) -> Result<NonZeroI32> {
    match Error::try_new(ret) {
        Err(ret) => Ok(ret),
        Ok(err) => Err(err),
    }
}

pub fn cvt_nz(error: libc::c_int) -> Result<()> {
    if error == 0 {
        Ok(())
    } else {
        Err(Error::new(error))
    }
}
