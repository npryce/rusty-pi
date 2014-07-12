
use std::io::timer::Timer;
use pi::gpio::{open_pin,Pin,In,Out,BothEdges};
use pi::epoll::{IoSelector,IN,ERR,ET};

mod pi;


fn poll_changes(mut button_pin : Pin, out: SyncSender<uint>) {
    button_pin.set_interrupt(BothEdges).unwrap();
    
    let mut selector = IoSelector::create().unwrap();
    selector.add(&button_pin, IN|ERR|ET, 1).unwrap();
    
    out.send(button_pin.get_value().unwrap());    
    loop {
        selector.wait().unwrap();
        out.send(button_pin.get_value().unwrap());
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
