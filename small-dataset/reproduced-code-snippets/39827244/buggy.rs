mod case2 {
    struct Foo {}

    struct Bar2<'b> {
        x: &'b Foo,
    }

    impl<'b> Bar2<'b> {
        fn f(&'b mut self) -> &'b Foo {
            self.x
        }
    }

    fn f4() {
        let foo = Foo {};
        let mut bar2 = Bar2 { x: &foo };
        bar2.f(); 
        let _z = bar2.f();
    }
}

fn main() {}
