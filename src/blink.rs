// On a rev 2 Pi with Quick2Wire board, run with ./button 18 5

use std::io::{IoResult};
use std::io::timer::sleep;
use pi::gpio::{open_pin,Out};

mod pi;

fn run(port : uint, count: uint) -> IoResult<()> {
    let mut pin = try!(open_pin(port));
    try!(pin.set_direction(Out));
    
    for i in range(0,count) {
        println!("{} / {}", i+1, count);
        
        try!(pin.set_value(1));
        sleep(1000);
        try!(pin.set_value(0));
        sleep(1000);
    }
    
    Ok(())
}

fn main() {
    let port = 18;
    let count = 10;
    
    match run(port, count) {
        Err(e) => {
            println!("error: {}", e);
        }
        Ok(_) => {}
    }
}
