trait Bar<T> {}

struct Foo<T> {
    data: Vec<Box<Bar<T>>>,
}

impl<T> Foo<T> {
    fn add<U: Bar<T>>(&mut self, x: U) {
        self.data.push(Box::new(x));
    }
}

fn main() { }