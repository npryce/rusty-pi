
#![allow(dead_code)]

extern crate libc;


#[allow(non_camel_case_types)]
mod ffi {
    use super::libc::{c_int, c_short, c_ushort};
    
    static M_TEN : c_ushort =     0x0010; // we have a ten bit chip address
    static M_RD : c_ushort =      0x0001;
    static M_NOSTART : c_ushort = 0x4000;
    static REV_DIR_ADDR : c_ushort =  0x2000;
    static IGNORE_NAK : c_ushort =    0x1000;
    static NO_RD_ACK : c_ushort =     0x0800;
    
    struct i2c_msg {
        addr : u16,      // slave address
        flags: c_ushort,  
        len: c_short,    // msg length
        buf: *mut u8     // pointer to msg data
    }

    struct i2c_rdwr_ioctl_data {
        msgs : *mut i2c_msg,   // pointers to i2c_msgs
        nmsgs : c_int
    }
}

