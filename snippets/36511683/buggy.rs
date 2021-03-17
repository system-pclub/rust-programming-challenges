struct Chain {
    n: u32,
}

impl Chain {
    fn new(start: u32) -> Chain {
        Chain { n: start }
    }
}

fn digit_factorial_sum(i: u32) -> u32 {
    i
}

impl Iterator for Chain {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.n = digit_factorial_sum(self.n);
        Some(self.n)
    }
}

fn main() {
    let mut v = Vec::with_capacity(100);
    let v = Chain::new(0)
        .inspect(|&x| {
            v.push(x)
        })
        .skip_while(|&x| {
            return v.contains(&x);
        });
}