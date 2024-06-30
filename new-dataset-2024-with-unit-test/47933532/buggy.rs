struct IntBorrower<'inner>(&'inner mut i32);

impl<'outer: 'inner, 'inner> IntBorrower<'inner> {
    fn int_mut(&'outer mut self) -> &'inner mut i32 {
        self.0
    }

    //this compiles
    fn do_something(&'outer mut self) {
        {
            *self.0 += 5;
        }

        {
            *self.0 += 5;
        }
    }

    //this does not compile
    fn do_something_but_with_getters(&'outer mut self) {
        {
            *self.int_mut() += 5;
        }

        {
            *self.int_mut() += 5;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_borrower() {
        let mut i = 0;
        let mut int_borrower = IntBorrower(&mut i);
        int_borrower.do_something();
        assert_eq!(i, 10);

        let mut i = 0;
        let mut int_borrower = IntBorrower(&mut i);
        int_borrower.do_something_but_with_getters();
        assert_eq!(i, 10);
    }
}
