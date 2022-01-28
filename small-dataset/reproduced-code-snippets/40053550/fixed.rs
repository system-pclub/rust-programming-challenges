trait Bar<T> {}

struct Foo<'a, T> {
    data: Vec<Box<dyn Bar<T> + 'a>>,
}

impl<'a, T> Foo<'a, T> {
    fn add<U>(&mut self, x: U)
    where
        U: Bar<T> + 'a,
    {
        self.data.push(Box::new(x));
    }
}


fn main() { }