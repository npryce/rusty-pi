
#![allow(dead_code)]

extern crate native;
extern crate libc;

use self::libc::{c_int,close};
pub use self::native::io::file::fd_t;
use std::io::{IoResult,IoError};

pub struct Fd {
    pub native: fd_t
}

impl Fd {
    pub fn own(fd: fd_t) -> Fd {
        Fd{native: fd}
    }
}

impl Drop for Fd {
    fn drop(&mut self) {
        unsafe {
            close(self.native);
        }
    }
}

pub fn check_syscall<T>(status: c_int, result_fn: |c_int|-> T) -> IoResult<T> {
    if status < 0 {
        Err(IoError::last_error())
    }
    else {
        Ok(result_fn(status))
    }
}

pub fn check_syscall_action(status: c_int) -> IoResult<()> {
    check_syscall(status, |_|())
}
