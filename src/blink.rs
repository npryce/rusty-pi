
use std::io::timer::sleep;
use pi::gpio::{open_pin,Out};

mod pi;


fn main() {
    let mut pin = open_pin(18,Out).unwrap();
    let mut value = 1;
    
    for i in range(0u,10) {
        println!("{} / {}", i+1, 10u);
        
        pin.set_value(value).unwrap();
        value = 1 - value;
        sleep(1000);
    }
}
