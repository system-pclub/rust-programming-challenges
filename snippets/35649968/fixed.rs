struct Board {
    n: usize,
    cur:  Vec<u32>,
    next: Vec<u32>,
}

impl Board {
    fn new(size: usize) -> Board {
        Board {
            n: size,
            cur: vec![0;size],
            next: vec![0;size],
        }
    }

    fn swap(&mut self) -> &Board {
        std::mem::swap(&mut self.cur, &mut self.next);
        self
    }
    
}

fn main() {}