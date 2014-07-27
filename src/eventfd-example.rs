
use pi::eventfd::semaphore;
use pi::epoll::{IoSelector,IN};
use std::io::timer::sleep;

mod pi;

fn main() {
    let (send, recv) = semaphore(0).unwrap();
    
    spawn(proc() {
        println!("SENDER: sleeping...");
        sleep(1000);
        println!("SENDER: awake - sending signal");
        send.signal().unwrap();
    });
    
    spawn(proc() {
        let mut s = IoSelector::create().unwrap();
        s.add(&recv, IN, 0).unwrap();
        
        println!("RECEIVER: waiting for signal...");
        s.wait().unwrap();
        
        println!("RECEIVER: receiving signal");
        recv.recv().unwrap();
        println!("RECEIVER: signal received");
    });
}
