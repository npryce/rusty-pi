use std::io::File;
use std::io::timer::sleep;

fn main() {
    File::create(&Path::new("/sys/class/gpio/export")).write_str("18").unwrap();
    
    let mut pin_file = File::create(&Path::new("/sys/class/gpio/gpio18/value"));
    
    for i in range(1,21) {
        pin_file.write_str((i%2).to_str()).unwrap();
        sleep(500);
    }
    
    File::create(&Path::new("/sys/class/gpio/unexport")).write_str("18").unwrap();
}
