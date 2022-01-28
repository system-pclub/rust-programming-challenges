fn foo(s: &str) {
    let id = |x| x;
    println!("{}", id(s));
}

fn main() {
    foo("string");
}