// no compilation error


#[derive(Debug)]
struct Foo<'a>(std::marker::PhantomData<fn(&'a ())>);

fn hint<'a, Arg>(_: &'a Arg) -> Foo<'a> {
    Foo(std::marker::PhantomData)
}

fn check<'a>(_: &Foo<'a>, _: &'a ()) {}

fn main() {
    let outlived = ();
    let foo;

    {
        let shortlived = ();
        foo = hint(&shortlived);
        // error: `shortlived` does not live long enough
        // check(&foo, &shortlived);
    }

    check(&foo, &outlived);
}

