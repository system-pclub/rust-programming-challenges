struct Foo {
    data: Vec<Bar>,
    a: usize,
}

struct Bar {
    b: usize,
}

impl Foo {
    fn example(&mut self, c: usize) {
        let baz = &mut self.data[c];
        let z = self.calc(c);
        baz.b = 42 + z;
    }

    fn calc(&self, x: usize) -> usize {
        self.a * x
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
