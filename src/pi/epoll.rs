
#![allow(dead_code)]

extern crate libc;

use self::libc::{c_int,close};
use std::io::{IoResult,IoError};
use native::io::file::fd_t;

pub static CLOEXEC : u32 = 02000000;

static ADD: c_int = 1;
static DEL: c_int = 2;
static MOD: c_int = 3;

pub static IN: u32 = 0x01;
pub static PRI: u32 = 0x02;
pub static OUT: u32 = 0x04;
pub static ERR: u32 = 0x08;
pub static HUP: u32 = 0x10;
pub static ONESHOT: u32 = 0x40000000;
pub static ET: u32 = 0x80000000;

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


pub trait Pollable {
    fn fd(&self) -> fd_t;
}


#[deriving(Copy,Show)]
pub struct Event {
    id: uint,
    events: u32
}

pub struct Epollfd {
    fd: c_int
}

impl Drop for Epollfd {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

impl Epollfd {
    pub fn create() -> IoResult<Epollfd> {
        Epollfd::create1(0)
    }
    
    pub fn create1(flags: int) -> IoResult<Epollfd> {
        let fd = unsafe {
            epoll_create1(flags as c_int)
        };
        if fd < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(Epollfd{fd: fd})
    }
    
    pub fn add(&mut self, polled_fd: c_int, events: u32, id: uint) -> IoResult<()> {
        let mut ev = EpollEvent{events:events, data:id as u64};
        let ev_ptr: *mut EpollEvent = &mut ev;
        
        let status = unsafe {
            epoll_ctl(self.fd, ADD, polled_fd, ev_ptr)
        };
        if status < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(())
    }
    
    pub fn update(&mut self, polled_fd: c_int, events: u32, id: uint) -> IoResult<()> {
        let mut ev = EpollEvent{events:events, data:id as u64};
        let ev_ptr: *mut EpollEvent = &mut ev;
        
        let status = unsafe {
            epoll_ctl(self.fd, MOD, polled_fd, ev_ptr)
        };
        if status < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(())
    }
    
    pub fn remove(&mut self, polled_fd: c_int) -> IoResult<()> {
        let mut ev = EpollEvent{events:0, data:0};
        let ev_ptr: *mut EpollEvent = &mut ev;
        
        let status = unsafe {
            epoll_ctl(self.fd, DEL, polled_fd, ev_ptr)
        };
        if status < 0 {
            return Err(IoError::last_error())
        }
        
        Ok(())
    }
    
    pub fn wait(&mut self) -> IoResult<Event> {
        let mut ev = EpollEvent{events:0, data:0};
        let ev_ptr: *mut EpollEvent = &mut ev;
        
        let n_events = unsafe { 
            epoll_wait(self.fd, ev_ptr, 1 as c_int, -1 as c_int) 
        };
        if n_events < 0 {
            return Err(IoError::last_error());
        }
        
        Ok(Event{id: ev.data as uint, events: ev.events})
    }
}

/*
#[test]
fn test_epoll_create1() {
  assert create1(0) >= 0;
  assert create1(EPOLL_CLOEXEC) >= 0;
  assert create1(-1) == -1;
}

#[test]
fn test_epoll_ctl() {
  let epfd = create1(0);
  assert epfd >= 0;

  assert ctl(epfd, EPOLL_CTL_ADD, 0, {events:EPOLLIN, data:0u64}) == 0;
  assert ctl(epfd, EPOLL_CTL_ADD, 0, {events:EPOLLIN, data:0u64}) == -1;
  assert ctl(epfd, EPOLL_CTL_MOD, 0, {events:EPOLLOUT, data:0u64}) == 0;
  assert ctl(epfd, EPOLL_CTL_DEL, 0, {events:EPOLLIN, data:0u64}) == 0;

  assert ctl(epfd, EPOLL_CTL_ADD, -1, {events:EPOLLIN, data:0u64}) == -1;
  assert ctl(epfd, EPOLL_CTL_MOD, -1, {events:EPOLLIN, data:0u64}) == -1;
  assert ctl(epfd, EPOLL_CTL_DEL, -1, {events:EPOLLIN, data:0u64}) == -1;
}

#[test]
fn test_epoll_wait() {
  // add stdout to epoll set and wait for it to become writable
  // should be immediate, it's an error if we hit the 50 ms timeout
  let epfd = epoll_create1(0);
  assert epfd >= 0;

  let magic = 42u64;
  assert ctl(epfd, EPOLL_CTL_ADD, 1, {events:EPOLLOUT, data:magic}) == 0;
  assert ctl(epfd, EPOLL_CTL_ADD, 2, {events:EPOLLOUT, data:magic}) == 0;

  let events: [mut EpollEvent] = [
    mut {events:0i32, data:0u64}, {events:0i32, data:0u64}];

  let n = epoll_wait(epfd, events, 50);
  assert n == 2;
  assert events[0].data == magic;
  assert events[0].events & EPOLLOUT == EPOLLOUT;
}
*/

