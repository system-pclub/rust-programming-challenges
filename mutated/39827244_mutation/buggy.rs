mod case2 {
    struct Foo {}

    struct Bar2<'b> {
        x: &'b Foo,
    }

    impl<'b> Bar2<'b> {
        fn f(self) -> Self {
            self
        }
    }

    fn f4() {
        let foo = Foo {};
        let mut bar2 = Bar2 { x: &foo };
        bar2.f(); // (3) 'bar2' is borrowed as mutable, but who borrowed it?
        let z = bar2.f(); // error: cannot borrow `bar2` as mutable more than once 
                          // at a time [E0499]
    }
}

fn main() {}