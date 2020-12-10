struct Baz<T>(T);

impl<T> Baz<T> {
    fn call(&self) -> &T {
        &self.0
    }
}

fn make_baz<T>(t: T) -> Box<Baz<T>> {
    Box::new(Baz(t))
}

fn main() {}