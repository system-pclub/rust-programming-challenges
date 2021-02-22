enum Foo<T> {
    Bar(T),
    Baz(T),
}

impl<T> Foo<T> {
    fn switch(&mut self) {
        *self = match self {
            &mut Foo::Bar(val) => Foo::Baz(val),
            &mut Foo::Baz(val) => Foo::Bar(val),
        }
    }
}

fn main() {}