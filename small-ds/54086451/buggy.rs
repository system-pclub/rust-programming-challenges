struct Foo {
    num: u32
}

struct Bar<'a> {
    foo: Foo,
    num_ref: &'a u32,
}

fn foo<'a>() -> Bar<'a> {
    let f = Foo { num: 2 };
    let n: &'a u32 = &f.num;
    return Bar { foo: f, num_ref: n };
}

fn main() { }