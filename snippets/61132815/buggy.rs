use std::convert::TryFrom;

trait Foo: AsRef<[u8]> + TryFrom<&[u8]> {}

fn main() {}