
#![allow(dead_code)]

extern crate native;
extern crate libc;

use self::libc::c_int;
use std::io::IoResult;
use super::unixio::{Fd,check_syscall,check_syscall_action};

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
    fn fd(&self) -> c_int;
}

#[deriving(Copy,Show)]
pub struct Event {
    id: uint,
    events: u32
}

pub struct IoSelector {
    fd: Fd
}

impl IoSelector {
    pub fn create() -> IoResult<IoSelector> {
        IoSelector::create1(0)
    }
    
    pub fn create1(flags: int) -> IoResult<IoSelector> {
        check_syscall(unsafe {epoll_create1(flags as c_int)}, 
                      |fd| { IoSelector{fd: Fd::own(fd)} })
    }
    
    pub fn add<'a, T:IoEventSource>(&'a mut self, event_source: &'a T, events: u32, id: uint) -> IoResult<()> {
        self.ctl(ADD, event_source, events, id)
    }
    
    pub fn update<'a, T:IoEventSource>(&'a mut self, event_source: &'a T, events: u32, id: uint) -> IoResult<()> {
        self.ctl(MOD, event_source, events, id)
    }
    
    pub fn remove<'a, T:IoEventSource>(&'a mut self, event_source: &'a T) -> IoResult<()> {
        self.ctl(DEL, event_source, 0, 0)
    }
    
    fn ctl<T:IoEventSource>(&mut self, op: c_int, event_source: &T, events: u32, id: uint) -> IoResult<()> {
        let mut ev = EpollEvent{events:events, data:id as u64};
        check_syscall_action(unsafe {epoll_ctl(self.fd.native, op, event_source.fd(), &mut ev)})
    }
    
    pub fn wait(&mut self) -> IoResult<Event> {
        let mut ev = EpollEvent{events:0, data:0};
        
        check_syscall(unsafe {epoll_wait(self.fd.native, &mut ev, 1 as c_int, -1 as c_int)},
                      |_| Event{id: ev.data as uint, events: ev.events})
    }
}
