// On a rev 2 Pi with Quick2Wire board, run with ./button 18 5

use std::io::{IoResult};
use std::io::timer::sleep;
use std::os::args;
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

fn opt<'a>(args : &'a Vec<~str>, i : uint) -> Option<&'a ~str> {
    if i < args.len() {
        Some(args.get(i))
    } else {
        None
    }
}

fn main() {
    let a = args();
    let port : uint = opt(&a, 1).and_then(|s|{from_str::<uint>(*s)}).unwrap_or(18);
    let count : uint = opt(&a, 2).and_then(|s|{from_str::<uint>(*s)}).unwrap_or(10);
    
    match run(port, count) {
        Err(e) => {
            println!("error: {}", e);
        }
        Ok(_) => {}
    }
}
