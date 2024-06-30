use std::ops::Add;

#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl<'a> Add for &'a Complex {
    type Output = Complex;

    fn add(self, other: &'a Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

fn task(x: f64, y: f64) -> (f64, f64) {
    let c: Complex = Complex { re: x, im: y};
    let d = &c + &c;
    (d.re, d.im)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = task(1.0, 2.0);
        assert_eq!(t, (2.0, 4.0));
    }
}