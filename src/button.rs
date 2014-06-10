// On a rev 2 Pi with Quick2Wire board, run with ./button 17 5

use std::io::{IoResult};
use std::io::timer::sleep;
use pi::gpio::{open_pin,In};

mod pi;

fn run(port : uint, count: uint) -> IoResult<()> {
    let mut pin = try!(open_pin(port));
    try!(pin.set_direction(In));
    
    for i in range(0,count) {        
        let v = try!(pin.get_value());
        println!("{} / {}, value = {}", i+1, count, v);
        sleep(1000);
    }
    
    Ok(())
}

fn main() {
    let port : uint = 23;
    let count : uint = 10;
    
    match run(port, count) {
        Err(e) => {
            println!("error: {}", e);
        }
        Ok(_) => {}
    }
}
