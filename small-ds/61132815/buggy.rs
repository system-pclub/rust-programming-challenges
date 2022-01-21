use std::convert::TryFrom;

trait Foo: AsRef<[u8]> + TryFrom<&[u8]> {}
// nor
// trait Foo<'a>: AsRef<[u8]> + TryFrom<&'a [u8]> {}

fn main() {}