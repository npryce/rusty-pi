
fn main() {
    let ibox = box 10;
    use_boxed_value(ibox);
    println!("finished with {}", ibox);
}

fn use_boxed_value(iref: &int) {
    println!("borrowing {}", iref);
}
