
use pi::eventfd::semaphore;
use std::io::timer::sleep;

mod pi;

fn main() {
    let (send, recv) = semaphore(0).unwrap();
    
    spawn(proc() {
        sleep(1000);
        println!("sending signal");
        send.signal().unwrap();
    });
    
    spawn(proc() {
        recv.recv().unwrap();
        println!("signal received");
    });
}
