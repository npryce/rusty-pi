
#![allow(dead_code)]

pub mod gpio {
    use std::fmt::Show;
    use std::io::{File,Open,Write,ReadWrite,SeekSet,IoResult,IoError,OtherIoError};
    
    #[deriving(Show)]
    pub enum Direction {In, Out}
    impl Direction {
        fn to_gpio(self) -> &'static str {
            match self {
                In => "in",
                Out => "out"
            }
        }
    }
    
    #[deriving(Show)]
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
    
    #[deriving(Copy)]
    pub struct Pin {
        port : uint,
        file : File
    }
    
    fn write_value_to<T:Show>(path: &str, value: T) -> IoResult<()> {
        let mut f = try!(File::open_mode(&Path::new(path), Open, Write));
        write!(f, "{}", value)
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
            try!(self.file.seek(0, SeekSet));
            let value_str = try!(self.file.read_to_str());
            
            match from_str::<uint>(value_str.as_slice().trim()) {
                Some(value) => Ok(value),
                None => Err(IoError {
                    kind: OtherIoError,
                    desc: "unexpected value read from GPIO file",
                    detail: None
                })
            }
        }
        
        pub fn set_value(&mut self, value : uint) -> IoResult<()> {
            try!(write!(self.file, "{}", value));
            self.file.flush()
        }
    }
    
    impl Drop for Pin {
        fn drop(&mut self) {
            drop(write_value_to("/sys/class/gpio/unexport", self.port));
        }
    }
    
    pub fn open_pin(port: uint) -> IoResult<Pin> {
        try!(write_value_to("/sys/class/gpio/export", port));
        let pin_path = format!("/sys/class/gpio/gpio{:u}/value", port);
        let pin_file = try!(File::open_mode(&Path::new(pin_path), Open, ReadWrite));
        Ok(Pin{port:port, file:pin_file})
    }
}
