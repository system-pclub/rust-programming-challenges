pub struct Foo<'a, T> {
    cb: Box<dyn FnMut(Vec<T>) + 'a>,
}

impl<'a, T> Foo<'a, T> {
    pub fn new<F: FnMut(Vec<T>) + 'a>(cb: F) -> Self {
        Self { cb: Box::new(cb) }
    }

    pub fn fun(&mut self, v: T) {
        let vector = vec![v];
        (self.cb)(vector);
    }
}

fn main() {
    let mut a = Vec::new();
    {
        let mut foo = Foo::new(|v| {
            for i in v {
                a.push(i);
            }
        });
        foo.fun(1);
    }
    println!("{:?}", a);
}