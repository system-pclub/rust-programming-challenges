use std::convert::TryFrom;

trait Foo: AsRef<[u8]> + for<'a> TryFrom<&'a [u8]> {}
fn main() {}


#[cfg(test)]
mod tests {
    use super::*;
    struct Bar {
        data: Vec<u8>,
    }

    impl AsRef<[u8]> for Bar {
        fn as_ref(&self) -> &[u8] {
            &self.data
        }
    }

    impl<'a> TryFrom<&'a [u8]> for Bar {
        type Error = ();

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            Ok(Bar { data: value.to_vec() })
        }
    }

    impl Foo for Bar {}

    fn foo<T: Foo>(_: T) {}

    #[test]
    fn test_foo() {
        let bar = Bar { data: vec![1, 2, 3] };
        foo(bar);
    }
}