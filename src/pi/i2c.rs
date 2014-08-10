
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate libc;

use std::mem::transmute;
use std::io::IoResult;
use self::libc::{c_int, c_ushort, open, O_RDWR};
use super::unixio::{Fd, check_syscall, check_syscall_action};

    
static M_TEN : c_ushort =         0x0010; // we have a ten bit chip address
static M_RD : c_ushort =          0x0001;
static M_NOSTART : c_ushort =     0x4000;
static REV_DIR_ADDR : c_ushort =  0x2000;
static IGNORE_NAK : c_ushort =    0x1000;
static NO_RD_ACK : c_ushort =     0x0800;

static IOCTL_RETRIES : c_int =     0x0701; // number of times a device address should be polled when not acknowledging
static IOCTL_TIMEOUT : c_int =     0x0702; // set timeout in units of 10 ms
// NOTE: Slave address is 7 or 10 bits, but 10-bit addresses are NOT supported! (due to code brokenness)
static IOCTL_SLAVE : c_int =       0x0703; //Use this slave address
static IOCTL_SLAVE_FORCE : c_int = 0x0706; // Use this slave address, even if it is already in use by a driver!
static IOCTL_FUNCS : c_int =       0x0705; // Get the adapter functionality mask
static IOCTL_RDWR : c_int =        0x0707; // Combined R/W transfer (one STOP only)
static IOCTL_PEC : c_int =         0x0708; // != 0 to use PEC with SMBus
static IOCTL_SMBUS : c_int =       0x0720; // SMBus transfer

pub type SlaveAddress = u16;

struct i2c_msg<'a> {
    addr : SlaveAddress,
    flags: u16,
    len: u16,            // msg length
    buf: *mut u8         // pointer to msg data
}

struct i2c_rdwr_ioctl_data<'a> {
    msgs : *mut i2c_msg<'a>,   // pointers to i2c_msgs
    nmsgs : u32
}

extern {
    fn ioctl(fd: c_int, req: c_int, ...) -> c_int;
}


#[deriving(Copy)]
pub enum Message<'a> {
    Read(&'a mut[u8]),
    Write(&'a [u8])
}

pub struct Master {
    fd: Fd
}

impl Master {
    pub fn open(bus_index: uint) -> IoResult<Master> {
        check_syscall(unsafe {open(format!("/dev/i2c-{:u}", bus_index).to_c_str().as_ptr(), O_RDWR, 0)},
                      |fd| { Master{fd: Fd::own(fd)} })
    }
    
    pub fn transaction<'a>(&self, addr: SlaveAddress, messages: &'a[Message]) -> IoResult<()> {
        unsafe {
            let mut i2c_msgs = Vec::from_fn(messages.len(), |i| {
                match messages[i] {
                    Read(ref buf) =>
                        i2c_msg{addr: addr, flags: M_RD, 
                                buf: transmute(buf.as_ptr()),
                                len: buf.len() as u16},
                    Write(ref buf) =>
                        i2c_msg{addr: addr, flags: 0, 
                                buf: transmute(buf.as_ptr()),
                                len: buf.len() as u16}
                }
            });
            
            check_syscall_action(
                ioctl(self.fd.native, IOCTL_RDWR, 
                      &i2c_rdwr_ioctl_data{msgs: i2c_msgs.as_mut_ptr(), nmsgs: i2c_msgs.len() as u32}))
        }
    }
}

pub struct Slave<'a> {
    master: &'a Master, 
    addr: SlaveAddress
}

impl <'a> Slave<'a> {
    pub fn transaction(&'a self, messages: &[Message]) -> IoResult<()> {
        self.master.transaction(self.addr, messages)
    }
}
