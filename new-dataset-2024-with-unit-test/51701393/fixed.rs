use std::cell::RefCell;
use std::cmp::PartialOrd;

type Link<T> = RefCell<Option<Box<Node<T>>>>;

struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

struct Tree<T> {
    root: Link<T>,
}

impl<T: PartialOrd> Node<T> {
    fn new(val: T) -> Self {
        Node {
            key: val,
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }

    fn add(&self, val: T) {
        let mut branch = if val < self.key {
            self.left.borrow_mut()
        } else {
            self.right.borrow_mut()
        };
        if let Some(next) = &*branch {
            next.add(val);
            return;
        }
        //Separated from the if let so that branch is not borrowed.
        *branch = Some(Box::new(Node::new(val)));
    }

    fn find(&self, val: T) -> bool {
        if self.key == val {
            return true;
        }
        let branch = if val < self.key {
            self.left.borrow()
        } else {
            self.right.borrow()
        };
        if let Some(next) = &*branch {
            next.find(val)
        } else {
            false
        }
    }
}

impl<T: PartialOrd> Tree<T> {
    fn new() -> Self {
        Tree {
            root: RefCell::new(None),
        }
    }

    fn add(&self, val: T) {
        if let Some(root) = &*self.root.borrow() {
            root.add(val);
            return;
        }
        *self.root.borrow_mut() = Some(Box::new(Node::new(val)));
    }

    fn find(&self, val: T) -> bool {
        if let Some(root) = &*self.root.borrow() {
            root.find(val)
        } else {
            false
        }
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let tree = Tree::new();
        tree.add(5);
        tree.add(3);
        tree.add(7);
        assert_eq!(tree.find(5), true);
        assert_eq!(tree.find(3), true);
        assert_eq!(tree.find(7), true);
        assert_eq!(tree.find(4), false);
    }
}
