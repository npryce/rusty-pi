
use std::io::timer::{Timer,sleep};
use pi::gpio::{open_pin,Pin,In,Out};

mod pi;


fn poll_changes(mut button_pin : Pin, out: SyncSender<uint>) {
    let mut last_button_state = button_pin.get_value().unwrap();
    
    out.send(last_button_state);
    
    loop {
        let button_state = button_pin.get_value().unwrap();
        if button_state != last_button_state {
            out.send(button_state);
            last_button_state = button_state;
        }
        
        sleep(100);
    }
}


fn blink(button: Receiver<uint>, mut led_pin : Pin) {
    let mut timer = Timer::new().unwrap();    
    
    loop {
        loop {
            let mut button_state = button.recv();
            
            if button_state == 1 {
                let mut led_state = 1u;
                let blink_timeout = timer.periodic(1000);
                
                led_pin.set_value(led_state).unwrap();
                
                while button_state == 1 {
                    select! {
                        b = button.recv() => {button_state = b;},
                        _ = blink_timeout.recv() => {led_state = 1 - led_state;}
                    }
                    
                    led_pin.set_value(led_state*button_state).unwrap();
                }
            }
        }
    }
}

 
fn main() {
    let button_pin = open_pin(23,In).unwrap();
    let led_pin = open_pin(18,Out).unwrap();
    let (send, recv) = sync_channel(0);
    
    spawn(proc() { poll_changes(button_pin, send); });
    spawn(proc() { blink(recv, led_pin); });
}
