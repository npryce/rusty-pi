
use pi::i2c::{BusMaster,Read,Write};
mod pi;


fn main() {
    let bus = BusMaster::open(0).unwrap();
    
    let mut rd_buf = [0, ..16];
    
    bus.transaction(0x20, [
        Write([0x10, 0x00, 0x01]),
        Read(rd_buf.as_mut_slice())]).unwrap();
}
