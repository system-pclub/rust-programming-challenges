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
        let tmp = self.cur;
        self.cur = self.next;
        self.next = tmp;
        self
    }
    
}


fn main() {}