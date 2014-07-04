use std::fmt::Show;
use std::io::{File, IoResult};
use std::io::timer::sleep;

fn write_to<T:Show>(path: &str, value: T) -> IoResult<()> {
    let mut f = try!(File::create(&Path::new(path)));
    write!(f, "{}", value)
}

fn main() {
    write_to("/sys/class/gpio/export", 18u)
        .unwrap();

    write_to("/sys/class/gpio/gpio18/direction", "out")
        .unwrap();
    
    for i in range(1u, 21) {
        write_to("/sys/class/gpio/gpio18/value", i%2)
            .unwrap();
        sleep(500);
    }
    
    write_to("/sys/class/gpio/unexport", 18u)
        .unwrap();
}
