
extern crate green;
extern crate rustuv;

use std::io::signal::{Listener, Interrupt};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
    let mut listener = Listener::new();
    
    match listener.register(Interrupt) {
        Err(e) => {
            println!("failed to register for interrupt signal: {}", e);
            fail!();
        },
        _ => ()
    }
    
    loop {
        match listener.rx.recv() {
            Interrupt => {
                fail!("interrupted");
            },
            _ => ()
        }
    }
}
