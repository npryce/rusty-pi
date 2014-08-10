

use std::io::timer::sleep;
use pi::i2c::{Master,Read,Write};
mod pi;

fn main() {
    let bus = Master::open(1).unwrap();
    
    let mut rd_buf = [0u8, ..2];
    
    loop {
        bus.transaction(0x48, [
            Write([0x00]), // select channel 0
            Read(rd_buf.as_mut_slice())]).unwrap();
        
        println!("{}", rd_buf[1]);
        
        sleep(500);
    }
}
