#![allow(unused)]

struct Cursor {
    offset: usize,
    data: [u8; 4],
}
impl Cursor {
    fn read_slice(&mut self, n: usize) -> &[u8] {
        let data = &self.data[self.offset..self.offset + n];
        self.offset += n;
        data
    }
}
struct FooBar<'a> {
    foo: &'a [u8],
    bar: &'a [u8],
}

fn read_foobar_from<'a>(cursor: &'a mut Cursor) -> FooBar<'a> {
    FooBar {
        foo: cursor.read_slice(2),
        bar: cursor.read_slice(2),
    }
}

fn main() {}

fn create_cursor(data: &[u8; 4]) -> Cursor {
    Cursor { offset: 0, data: data.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_foobar_from() {
        let data = [1, 2, 3, 4];
        let mut cursor = create_cursor(&data);
        let foobar = read_foobar_from(&mut cursor);
        assert_eq!(foobar.foo, &[1, 2]);
        assert_eq!(foobar.bar, &[3, 4]);
    }
}
