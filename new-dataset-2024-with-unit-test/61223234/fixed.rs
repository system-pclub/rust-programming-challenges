struct StackLike<'a, X> {
    data: &'a mut [X],
}

impl<'a, X> StackLike<'a, X> {
    pub fn pop(&mut self) -> Option<&'a mut X> {
        let data = std::mem::replace(&mut self.data, &mut []);
        if let Some((last, subslice)) = data.split_last_mut() {
            self.data = subslice;
            Some(last)
        } else {
            None
        }
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
