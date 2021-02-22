fn helloworld(f: &Fn(f64)) {
    f(42f64);
}
 
pub fn main() {
    let mut killer = 2;
    helloworld(&|n| {
        println!("{}", n);
        killer += 1;
    });
}
