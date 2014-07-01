use std::io::File;

fn main() {
    let pin = 18;
    
    let mut export_file = File::create(&Path::new("/sys/class/gpio/export"));
    write!(export_file, "{}", pin);
}
