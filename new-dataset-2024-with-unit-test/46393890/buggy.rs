struct Test<'a> {
    a: &'a str,
    counter: u32,
}

impl<'a> Test<'a> {
    pub fn new() -> Self {
        Test { a: &mut "test", counter: 0}
    }

    pub fn dostuff(&'a mut self) {
        self.a = "new_string";
    }

    pub fn fixme(&'a mut self) {
        let mut i = 0;
        while i < 10 {
            self.dostuff();
            i += 1;
            self.counter += 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let mut test = Test::new();
        test.fixme();
        assert_eq!(test.a, "new_string");
        assert_eq!(test.counter, 10);
    }
}