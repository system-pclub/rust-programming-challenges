use std::vec::*;

struct Pool {
    strings: Vec<String>,
}

impl Pool {
    pub fn new() -> Self {
        Self { strings: vec![] }
    }

    pub fn some_f(&mut self) -> Vec<String> {
        let mut v = vec![];

        for i in 1..10 {
            let string = format!("{}", i);
            self.new_string(string);
            let string = &self.strings[i - 1];
            v.push(string[..].to_string());
        }

        v
    }

    fn new_string(&mut self, string: String) {
        self.strings.push(string);
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool() {
        let mut pool = Pool::new();
        let strings = pool.some_f();
        assert_eq!(strings, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    }
}