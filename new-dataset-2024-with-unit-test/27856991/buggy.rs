use std::ops::Add;

#[derive(Debug)]
struct BigInt {
    buf: Vec<i32>,
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let mut ret = self.buf.clone();
        ret.extend(other.buf.clone());
        BigInt { buf: ret }
    }
}

fn task(x: i32, y: i32) -> usize {
    let bi: BigInt = BigInt {buf: vec![x, y]};
    let bi2 = bi + bi;
    bi2.buf.len()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = task(1, 2);
        assert!(t >= 2);
    }
}