use std::convert::TryFrom;

trait Foo: AsRef<[u8]> + for<'a> TryFrom<&'a [u8]> {}

fn main() {}