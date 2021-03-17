struct Chain {
    n: u32,
    v: Vec<u32>
}

impl Chain {
    fn new(start: u32) -> Chain {
        Chain { n: start, v: vec![start] }
    }
}

fn digit_factorial_sum(i: u32) -> u32 {
    i
}

impl Iterator for Chain {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.v.push(self.n); // update lifetime
        self.n = digit_factorial_sum(self.n); // calculate n
        if self.v.contains(&self.n) { None } // check the lifetime vector for a duplicate
        else { Some(self.n) } // return n
    }
}

fn main() {
    let v = Chain::new(0);
}