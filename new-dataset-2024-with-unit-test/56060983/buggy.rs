struct MyStruct<'a> {
    s: &'a str,
    count: i32,
}

impl<'a> MyStruct<'a> {
    fn foo(&'a mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut m = MyStruct { s: "aaa", count: 0 };

    m.foo();
    m.foo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut m = MyStruct { s: "aaa", count: 0 };

        m.foo();
        m.foo();

        assert_eq!(m.count, 2);
    }
}