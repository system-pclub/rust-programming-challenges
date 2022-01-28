use owning_ref::BoxRef;

struct Foo {
    num: u32
}

fn foo<'a>() -> BoxRef<Foo, u32> {
    let f = BoxRef::new(Box::new(Foo { num: 2 }));
    let f = f.map(|f| &f.num);
    return f;
}

fn main() { }