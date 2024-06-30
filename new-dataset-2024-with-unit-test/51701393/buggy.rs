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
        let mut next = self.root.borrow();
        let node = Box::new(Node::new(val));
        match next.as_ref() {
            None => {
                self.root.replace(Some(node));
                ()
            }
            Some(root_ref) => {
                let mut prev = root_ref;
                let mut cur: Option<&Box<Node<T>>> = Some(root_ref);
                while let Some(node_ref) = cur {
                    prev = node_ref;
                    if node.key < node_ref.key {
                        next = node_ref.left.borrow();
                    } else {
                        next = node_ref.right.borrow();
                    }
                    cur = next.as_ref();
                }
                if node.key < prev.key {
                    prev.left.replace(Some(node));
                } else {
                    prev.right.replace(Some(node));
                }
            }
        }
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
