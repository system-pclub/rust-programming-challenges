struct Foo {
    data: Vec<Bar>,
    a: usize,
}

struct Bar {
    b: usize
}

impl Foo {
    fn example(&mut self, c: usize) {
        let baz = &mut self.data[c];
        // This avoids borrowing `self` in its entirety...
        let z = Self::calc(self.a, c);
        baz.b = 42 + z;
    }

    fn calc(a: usize, x: usize) -> usize {
        a * x
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut foo = Foo {
            data: vec![Bar { b: 0 }, Bar { b: 0 }],
            a: 2,
        };
        foo.example(1);
        assert_eq!(foo.data[1].b, 44);
    }
}