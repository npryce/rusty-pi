
#![allow(dead_code)]

extern crate libc;

use self::libc::{c_int,c_uint};
use std::io::IoResult;
use std::sync::Arc;
use super::unixio::{Fd,check_syscall,check_syscall_action};
use super::epoll::{IoEventSource,fd_t};

pub static SEMAPHORE : c_int = 1;
pub static CLOEXEC : c_int = 02000000;
pub static NONBLOCK : c_int = 04000;

pub struct Eventfd {
    fd: Fd
}

pub type EventCount = u64;
    
extern {
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn eventfd_read(fd: c_int, value: *mut EventCount) -> c_int;
    fn eventfd_write(fd: c_int, value: EventCount) -> c_int;
}

impl Eventfd {
    pub fn create(flags: c_int) -> IoResult<Eventfd> {
        check_syscall(unsafe {eventfd(0, flags)}, |fd| Eventfd{fd: Fd::own(fd)})
    }
    
    pub fn write(&self, n: EventCount) -> IoResult<()> {
        check_syscall_action(unsafe {eventfd_write(self.fd.native, n)})
    }
    
    pub fn read(&self) -> IoResult<EventCount> {
        let mut count : EventCount = 0;
        
        check_syscall(unsafe {eventfd_read(self.fd.native, &mut count)},
                      |_| count)
    }
}

pub struct Semaphore {
    fdref: Arc<Eventfd>
}

pub struct SemaphoreSender {
    fdref: Arc<Eventfd>
}

pub fn semaphore(flags: c_int) -> IoResult<(SemaphoreSender, Semaphore)> {
    Eventfd::create(SEMAPHORE|flags)
        .map(Arc::new)
        .map({|r| (SemaphoreSender{fdref:r.clone()}, Semaphore{fdref:r})})
}

impl Semaphore {
    pub fn recv(&self) -> IoResult<()> {
        (*self.fdref).read().and(Ok(()))
    }
}

impl IoEventSource for Semaphore {
    fn fd(&self) -> fd_t {
        self.fdref.fd.native
    }
}

impl SemaphoreSender {
    pub fn signal(&self) -> IoResult<()> {
        (*self.fdref).write(1)
    }
}

impl Clone for SemaphoreSender {
    fn clone(&self) -> SemaphoreSender {
        SemaphoreSender{fdref:self.fdref.clone()}
    }
}
