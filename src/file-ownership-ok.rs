
use std::io::{File,IoResult};

fn main() {
    let mut f = File::open(&Path::new("hello.txt"))
        .ok().expect("could not open file");
    use_file(&mut f)
        .ok().expect("could not use file");
    f.flush()
        .ok().expect("could not flush file");
}

fn use_file(f: &mut File) -> IoResult<()> {
    f.write_str("hello")
}
