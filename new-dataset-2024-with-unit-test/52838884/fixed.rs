trait Foo<'a> {
    fn get(&self) -> &i32;
}

struct Bar {
    data: i32,
}

impl<'a> Foo<'a> for Bar {
    fn get(&self) -> &i32 {
        &self.data
    }
}

struct Baz {
    data: i32,
}

impl<'a> Foo<'a> for Baz {
    fn get(&self) -> &i32 {
        &self.data
    }
}

fn get_foo(foo: &str) -> Box<dyn Foo> {
    let data = 15;
    if foo.starts_with("bar") {
        Box::new(Bar { data: data + 1 })
    } else {
        Box::new(Baz { data: data + 42 })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let foo = get_foo("bar+0");
        assert_eq!(*foo.get(), 16);
        let foo = get_foo("baz+0");
        assert_eq!(*foo.get(), 57);
    }
}