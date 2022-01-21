use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T> Iterator for PermutationIterator<T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Vec<&T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(self.vs[i].get(index).unwrap());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}