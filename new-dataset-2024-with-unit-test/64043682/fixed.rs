use std::cmp::{Ord, Ordering};
use std::fmt::Display;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Display + Ord> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    fn print_inorder(&self) {
        print!("[");
        self.inorder(|x| print!("{}, ", x));
        print!("]\n")
    }

    fn inorder(&self, f: fn(&T)) {
        if let Some(ref x) = self.left {
            (*x).inorder(f);
        }
        f(&self.value);
        if let Some(ref x) = self.right {
            (*x).inorder(f);
        }
    }

    fn insert(&mut self, val: T) {
        let mut node = self;

        loop {
            if val == node.value {
                return;
            }

            let child = match val.partial_cmp(&node.value).expect("Key comparison failed") {
                Ordering::Less => &mut node.left,
                Ordering::Equal => return,
                Ordering::Greater => &mut node.right,
            };

            match child {
                Some(ref mut c) => node = c,
                None => {
                    *child = Some(Box::new(Node::new(val)));
                    return;
                }
            }
        }
    }

    fn delete(mut this: Box<Node<T>>, target: &T) -> Option<Box<Node<T>>> {
        if target < &this.value {
            if let Some(left) = this.left.take() {
                this.left = Self::delete(left, target);
            }
            return Some(this);
        }

        if target > &this.value {
            if let Some(right) = this.right.take() {
                this.right = Self::delete(right, target);
            }
            return Some(this);
        }

        assert!(target == &this.value, "Faulty Ord implementation for T");

        match (this.left.take(), this.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.rightmost_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    Some(rightmost)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            }
        }
    }

    //  Returns the rightmost child, unless the node itself is that child.
    fn rightmost_child(&mut self) -> Option<Box<Node<T>>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}

struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Display> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, val: T) {
        match self.root {
            Some(ref mut n) => n.insert(val),
            None => self.root = Some(Box::new(Node::new(val))),
        }
    }

    fn print(&self) {
        match self.root {
            Some(ref n) => n.print_inorder(),
            None => println!("[]"),
        }
    }

    fn delete(&mut self, target: &T) {
        if let Some(root) = self.root.take() {
            self.root = Node::delete(root, target);
        }
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut t = Tree::<i64>::new();
        t.insert(14);
        t.insert(7);
        t.insert(21);
        assert_eq!(t.root.as_ref().unwrap().value, 14);
        assert_eq!(t.root.as_ref().unwrap().left.as_ref().unwrap().value, 7);
        assert_eq!(t.root.as_ref().unwrap().right.as_ref().unwrap().value, 21);
        t.delete(&21);
        assert_eq!(t.root.as_ref().unwrap().value, 14);
        assert_eq!(t.root.as_ref().unwrap().left.as_ref().unwrap().value, 7);
        assert!(t.root.as_ref().unwrap().right.is_none());
    }
}