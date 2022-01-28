use std::sync::Arc;

struct A<'a, T> {
    f: Box<dyn Fn(&u32) -> T + 'a>
}

struct B<'a> {
    inner: A<'a, Z<'a>>
}

impl<'a, T> A<'a, T> {
    fn new<F>(f: F) -> Self where F: Fn(&u32) -> T + 'a {
        A { f: Box::new(f) }
    }
}

struct X<'a> {
    _d: &'a std::marker::PhantomData<()>
}

struct Z<'a> {
    _d: &'a std::marker::PhantomData<()>
}

impl<'a> X<'a> {
    fn g(&self, y: u32) -> Z {
        Z { _d: &std::marker::PhantomData }
    }
}

impl<'a> B<'a> {
    fn new(x: Arc<X<'a>>) -> Self {
        B {
            inner: A::new(move |y: &u32| -> Z {
                x.g(*y)
            })
        }
    }
}

fn main() {
}