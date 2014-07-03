
fn main() {
    let mut ibox = box 10;
    use_boxed_value(ibox);
    println!("{}", ibox);
}

fn use_boxed_value(iref: &mut int) {
    *iref = 20;
}
