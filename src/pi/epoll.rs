
#![allow(dead_code)]

extern crate libc;

use self::libc::{c_int,close};
use std::io::{IoResult,IoError};
pub use native::io::file::fd_t;

pub static CLOEXEC : u32 = 02000000;

pub static IN: u32 = 0x01;
pub static PRI: u32 = 0x02;
pub static OUT: u32 = 0x04;
pub static ERR: u32 = 0x08;
pub static HUP: u32 = 0x10;
pub static ONESHOT: u32 = 0x40000000;
pub static ET: u32 = 0x80000000;

static ADD: c_int = 1;
static DEL: c_int = 2;
static MOD: c_int = 3;

#[packed]
struct EpollEvent {
    events: u32,
    data: u64
}

extern {
    fn epoll_create1(flags: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
}

pub trait IoEventSource {
    fn fd(&self) -> fd_t;
}

#[deriving(Copy,Show)]
pub struct Event {
    id: uint,
    events: u32
}

pub struct IoSelector {
    fd: c_int
}

impl IoSelector {
    pub fn create() -> IoResult<IoSelector> {
        IoSelector::create1(0)
    }
    
    pub fn create1(flags: int) -> IoResult<IoSelector> {
        let fd = unsafe {
            epoll_create1(flags as c_int)
        };
        
        if fd < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(IoSelector{fd: fd})
    }
    
    pub fn add<T:IoEventSource>(&mut self, event_source: &T, events: u32, id: uint) -> IoResult<()> {
        self.ctl(ADD, event_source, events, id)
    }
    
    pub fn update<T:IoEventSource>(&mut self, event_source: &T, events: u32, id: uint) -> IoResult<()> {
        self.ctl(MOD, event_source, events, id)
    }
    
    pub fn remove<T:IoEventSource>(&mut self, event_source: &T) -> IoResult<()> {
        self.ctl(DEL, event_source, 0, 0)
    }
    
    fn ctl<T:IoEventSource>(&mut self, op: c_int, event_source: &T, events: u32, id: uint) -> IoResult<()> {
        let mut ev = EpollEvent{events:events, data:id as u64};
        
        let status = unsafe {
            epoll_ctl(self.fd, op, event_source.fd(), &mut ev)
        };
        if status < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(())
    }
    
    pub fn wait(&mut self) -> IoResult<Event> {
        let mut ev = EpollEvent{events:0, data:0};
        
        let n_events = unsafe { 
            epoll_wait(self.fd, &mut ev, 1 as c_int, -1 as c_int) 
        };
        
        if n_events < 0 {
            return Err(IoError::last_error());
        }
        
        Ok(Event{id: ev.data as uint, events: ev.events})
    }
}

impl Drop for IoSelector {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}
