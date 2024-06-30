use std::ops::Add;

#[derive(Debug)]
struct BigInt {
    buf: Vec<i32>,
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let mut ret = self.buf.clone();
        ret.extend(other.buf);
        BigInt { buf: ret }
    }
}

fn main() {
    let bi: BigInt = BigInt {buf: vec![]};
    let bi2 = bi + bi;
}
