use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub enum FooBox<'a, T> {
    Ref(&'a Foo<T>),
    Own(Foo<T>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Foo<T> {
    data: T,
}

impl<T> Foo<T> {
    /// Construct a new Foo
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

pub trait Bar<C, P>
where
    C: Clone,
    P: Clone,
{
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a C, FooBox<'a, P>)> + 'a
    where
        C: 'a,
        P: 'a;
}

pub struct MyBar<'a, C, F0, F1, P0, P1>
where
    C: Clone,
    F0: Bar<C, P0>,
    F1: Bar<C, P1>,
    P0: Clone,
    P1: Clone,
{
    bar0: &'a F0,
    bar1: &'a F1,

    // Used for type checking at compile time
    c: PhantomData<C>,
    p0: PhantomData<P0>,
    p1: PhantomData<P1>,
}

impl<'a, C, F0, F1, P0, P1> Bar<C, (FooBox<'a, P0>, FooBox<'a, P1>)>
    for MyBar<'a, C, F0, F1, P0, P1>
where
    C: Clone + PartialOrd,
    F0: Bar<C, P0>,
    F1: Bar<C, P1>,
    P0: Clone + PartialEq,
    P1: Clone + PartialEq,
{
    fn iter<'b>(
        &'b self,
    ) -> impl Iterator<Item = (&C, FooBox<(FooBox<'a, P0>, FooBox<'a, P1>)>)> + 'b
    where
        C: 'b,
        P0: 'b,
        P1: 'b,
    {
        MyBarIter::new(self.bar0.iter(), self.bar1.iter())
    }
}

/// Iterator for an MyBar
pub struct MyBarIter<'a, C, I0, I1, P0, P1>
where
    C: Clone + PartialOrd + 'a,
    I0: Iterator<Item = (&'a C, FooBox<'a, P0>)>,
    I1: Iterator<Item = (&'a C, FooBox<'a, P1>)>,
    P0: Clone + PartialEq + 'a,
    P1: Clone + PartialEq + 'a,
{
    iter0: I0,
    iter1: I1,
}

impl<'a, C, I0, I1, P0, P1> MyBarIter<'a, C, I0, I1, P0, P1>
where
    C: Clone + PartialOrd,
    I0: Iterator<Item = (&'a C, FooBox<'a, P0>)>,
    I1: Iterator<Item = (&'a C, FooBox<'a, P1>)>,
    P0: Clone + PartialEq,
    P1: Clone + PartialEq,
{
    /// Construct a new iterator from the given MyBar
    pub fn new(iter0: I0, iter1: I1) -> Self {
        Self { iter0, iter1 }
    }
}

impl<'a, C, I0, I1, P0, P1> Iterator for MyBarIter<'a, C, I0, I1, P0, P1>
where
    C: Clone + PartialOrd,
    I0: Iterator<Item = (&'a C, FooBox<'a, P0>)>,
    I1: Iterator<Item = (&'a C, FooBox<'a, P1>)>,
    P0: Clone + PartialEq,
    P1: Clone + PartialEq,
{
    type Item = (&'a C, FooBox<'a, (FooBox<'a, P0>, FooBox<'a, P1>)>);

    fn next(&mut self) -> Option<Self::Item> {
        let next0 = self.iter0.next();
        let next1 = self.iter1.next();

        if let (Some((c0, p0)), Some((_, p1))) = (next0, next1) {
            return Some((c0, FooBox::Own(Foo::new((p0, p1)))));
        }

        return None;
    }
}

fn main() {
    println!("Hello, world!");
}
