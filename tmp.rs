fn helloworld(f: &mut FnMut(f64)) {
    f(42f64);
}

pub fn main() {
    let mut killer = 2;
    helloworld(&mut |n| {
        println!("{}", n);
        killer += 1;
    });
}