fn foo<'a, T: 'a>(t: T) -> Box<Fn() -> &'a T + 'a> {
    Box::new(move || &t)
}


fn main() {}