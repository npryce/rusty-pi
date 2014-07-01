
use std::io::File;

fn use_file(mut f: File) {
    // do something with f
}

fn main() {
    let mut f = File::open(&Path::new("hello.txt")).unwrap();
    use_file(f);
    f.flush();
}
