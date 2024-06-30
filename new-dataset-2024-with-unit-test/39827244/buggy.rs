mod case2 {
    struct Foo {
        x: i32,
    }
    struct Bar1 {
        x: Foo,
    }

    impl Bar1 {
        fn f<'a>(&'a mut self) -> &'a Foo {
            self.x.x += 1;
            &self.x
        }
    }

    // only for example
    pub fn f1(x: i32) -> String {
        let mut bar = Bar1 { x: Foo { x } };
        let y = bar.f(); // (1) 'bar' is borrowed by 'y'
        let z = bar.f(); // error (as expected) : cannot borrow `bar` as mutable more
                         // than once at a time [E0499]
        format!("{}", z.x)
    }

    pub fn f2(x: i32) -> String {
        let mut bar = Bar1 { x: Foo { x } };
        bar.f(); // (2) 'bar' is not borrowed after the call
        let z = bar.f(); // ok (as expected)
        format!("{}", z.x)
    }

    struct Bar2<'b> {
        x: &'b Foo,
        t: i32,
    }

    impl<'b> Bar2<'b> {
        fn f(&'b mut self) -> &'b Foo {
            self.t += 1;
            self.x
        }
    }

    pub fn f4(x: i32) -> String {
        let foo = Foo { x };
        let mut bar2 = Bar2 { x: &foo, t: x };
        bar2.f();
        let _z = bar2.f();
        format!("{}, {}", _z.x, bar2.t)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::case2::*;

    #[test]
    fn test_f1() {
        assert_eq!(f1(1), "3");
    }

    #[test]
    fn test_f2() {
        assert_eq!(f2(1), "3");
    }

    #[test]
    fn test_f4() {
        assert_eq!(f4(1), "1, 3");
    }
}