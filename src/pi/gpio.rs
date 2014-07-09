
#![allow(dead_code)]

extern crate libc;

use std::fmt::Show;
use std::io;
use std::io::{File,IoResult,IoError,TimedOut,ShortWrite,OtherIoError};
use std::rt::rtio;
use std::rt::rtio::{SeekSet,RtioFileStream};
use native::io::FileDesc;
use native::io::file::open;

    
#[deriving(Copy,Show)]
pub enum Direction {In, Out}

impl Direction {
    fn to_gpio(self) -> &'static str {
        match self {
            In => "in",
            Out => "out"
        }
    }
}

#[deriving(Copy,Show)]
pub enum Edge {NoInterrupt, RisingEdge, FallingEdge, BothEdges}

impl Edge {
    fn to_gpio(self) -> &'static str {
        match self {
            NoInterrupt => "none",
            RisingEdge => "rising",
            FallingEdge => "falling",
            BothEdges => "both"
        }
    }
}
    
pub struct Pin {
    port : uint,
    fd : FileDesc
}

fn write_value_to<T:Show>(path: &str, value: T) -> IoResult<()> {
    let mut f = try!(File::open_mode(&Path::new(path), io::Open, io::Write));
    write!(f, "{}", value)
}

// Had to copy & paste from std::io module because it's private
fn error_rtio_to_io(err: rtio::IoError) -> IoError {
    let rtio::IoError { code, extra, detail } = err;
    let mut ioerr = IoError::from_errno(code, false);
    ioerr.detail = detail;
    ioerr.kind = match ioerr.kind {
        TimedOut if extra > 0 => ShortWrite(extra),
        k => k,
    };
    return ioerr;
}


impl Pin {
    fn write_to_device_file<T:Show>(&mut self, filename: &str, value: T) -> IoResult<()> {
        write_value_to(format!("/sys/devices/virtual/gpio/gpio{:u}/{:s}", 
                               self.port, filename).as_slice(), value)
    }
    
    pub fn set_direction(&mut self, direction : Direction) -> IoResult<()> {
        self.write_to_device_file("direction", direction.to_gpio())
    }
    
    pub fn set_interrupt(&mut self, edge : Edge) -> IoResult<()> {
        self.write_to_device_file("edge", edge.to_gpio())
    }
    
    pub fn get_value(&mut self) -> IoResult<uint> {
        try!(self.fd.seek(0, SeekSet).map_err(error_rtio_to_io));
        
        let mut buf = [0u8];
        let amount = try!(self.fd.read(buf).map_err(error_rtio_to_io));
        
        if amount == 0 {
            Err(IoError {
                kind: OtherIoError,
                desc: "no value read from GPIO file",
                detail: None
            })
        }
        else if buf[0] == b'0' {
            Ok(0)
        }
        else if buf[0] == b'1' {
            Ok(1)
        }
        else {
            Err(IoError {
                kind: OtherIoError,
                desc: "unexpected value read from GPIO file",
                detail: Some(format!("{:u}", buf[0]))
            })
        }
    }
    
    pub fn set_value(&mut self, value : uint) -> IoResult<()> {
        let buf  = if value == 0 { b"0" } else { b"1" };
        
        self.fd.write(buf).map_err(error_rtio_to_io)
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        drop(write_value_to("/sys/class/gpio/unexport", self.port));
    }
}

pub fn open_pin(port: uint, direction: Direction) -> IoResult<Pin> {
    try!(write_value_to("/sys/class/gpio/export", port));
    
    let pin_path = format!("/sys/class/gpio/gpio{:u}/value", port);
    let pin_fd = try!(open(&pin_path.to_c_str(), rtio::Open, rtio::ReadWrite).map_err(error_rtio_to_io));
    
    let mut pin = Pin{port:port, fd:pin_fd};
    
    try!(pin.set_direction(direction));
    
    Ok(pin)
}
