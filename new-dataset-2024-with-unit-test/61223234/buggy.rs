struct StackLike<'a, X> {
    data: &'a mut [X],
}

impl<'a, X> StackLike<'a, X> {
    pub fn pop(&mut self) -> Option<&'a X> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        let result = &self.data[n - 1];
        self.data = &mut self.data[0..n - 1];
        Some(result)
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut data = [1, 2, 3, 4, 5];
        let mut stack = StackLike { data: &mut data };

        let x = stack.pop().unwrap();
        let y = stack.pop().unwrap();
        assert_eq!(*x, 5);
        assert_eq!(*y, 4);
    }
}
