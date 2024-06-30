use std::{cell::RefCell, rc::Rc};

enum Number {
    Pair(Rc<RefCell<Number>>, Rc<RefCell<Number>>),
    Value(u8),
}

fn propagate_left(mut node: Rc<RefCell<Number>>, val: u8) {
    loop {
        let mut ref_mut = node.borrow_mut();
        match (*ref_mut) {
            Number::Pair(left_node, _) => {
                node = left_node;
            }
            Number::Value(ref mut number_val) => {
                *number_val += val;
                return;
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagate_left() {
        let left = Rc::new(RefCell::new(Number::Value(1)));
        let right = Rc::new(RefCell::new(Number::Value(2)));
        let root = Rc::new(RefCell::new(Number::Pair(Rc::clone(&left), Rc::clone(&right))));

        propagate_left(Rc::clone(&root), 5);

        assert!(matches!(*left.borrow(), Number::Value(6)));
    }
}