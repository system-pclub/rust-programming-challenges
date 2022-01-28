fn foo(s: &str) {
    let id = |x: &str| x;
    println!("{}", id(s));
}

fn main() {
    foo("string");
}