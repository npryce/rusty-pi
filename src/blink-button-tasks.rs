
use std::io::timer::{Timer,sleep};
use pi::gpio::{open_pin,In,Out};

mod pi;


fn button_poller(out: SyncSender<uint>) {
    let mut button_pin = open_pin(23,In).unwrap();
    
    let mut last_button_state = 2u; // to guarantee the first button press is sent
    
    loop {
        let button_state = button_pin.get_value().unwrap();
        if button_state != last_button_state {
            out.send(button_state);
            last_button_state = button_state;
        }
        
        sleep(100);
    }
}


fn blinker(button: Receiver<uint>) {
    let mut led_pin = open_pin(18,Out).unwrap();
    let mut timer = Timer::new().unwrap();
    
    let blink_timeout = timer.periodic(1000);
    let mut led_state = 1u;
    let mut button_state = 0u;
    
    loop {
        select! {
            b = button.recv() => {button_state = b;},
            _ = blink_timeout.recv() => {led_state = 1 - led_state;}
        }
        
        led_pin.set_value(led_state*button_state).unwrap();
    }
}


fn main() {
    let (send, recv) = sync_channel(0);
    spawn(proc() { button_poller(send); });
    spawn(proc() { blinker(recv); });
}
