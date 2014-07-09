
use std::io::timer::sleep;
use pi::gpio::{open_pin,In,Out};

mod pi;


fn main() {
    let mut led_pin = open_pin(18,Out).unwrap();
    let mut button_pin = open_pin(23,In).unwrap();
    
    let mut led_value = 0u;
    
    for i in range(0u,10) {
        println!("{} / {}", i+1, 10u);
        
        let button_value = button_pin.get_value().unwrap();
        
        led_pin.set_value(led_value * button_value).unwrap();
        
        led_value = 1 - led_value;
        sleep(1000);
    }
}
