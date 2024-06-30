use std::{cmp::Ordering, rc::Rc, cell::RefCell, fmt::Debug};

#[derive(Debug, Clone)]
pub(crate) struct Tree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut node = &mut self.root;

        while let Option::Some(current_node) = node {
            let cmp = current_node.borrow().value.cmp(&value);
            match cmp {
                Ordering::Less => node = &mut current_node.borrow_mut().right,
                Ordering::Equal => return false,
                Ordering::Greater => node = &mut current_node.borrow_mut().left,
            };
        }

        *node = Option::Some(Rc::new(RefCell::new(Node {
            value,
            left: Option::None,
            right: Option::None,
        })));

        return true;
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(1), false);
    }
}