struct Foo<'a> {
    x: &'a mut i32,
}

impl<'b> Foo<'b> {
    fn x(&mut self) -> &mut i32 {
        *self.x += 1;
        self.x
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let y = &mut 5;
        let mut f = Foo { x: y };
        let t = f.x();
        *t += 1;
        assert_eq!(*f.x(), 8);
    }
}