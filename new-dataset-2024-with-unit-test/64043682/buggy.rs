use std::cmp::{Ordering, PartialOrd};
use std::fmt::Display;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Display + PartialOrd> Node<T> {
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
                None => { *child = Some(Box::new(Node::new(val))); return}
            }
        }
    }

    fn delete(&mut self, val: T) -> bool {
        if self.value == val {
            unimplemented!{"Cannot remove root (yet) :/"};
        }

        let mut node = self;

        loop {
            if val < node.value {
                if let Some(ref mut c) = node.left {

                    if c.value == val {
                        if c.left.is_none() && c.right.is_none() {
                            // Error: cannot assign to node.left here
                            node.left = None;
                        }
                        else if c.left.is_some() && c.right.is_none() {
                            // Error: again cannot assign to node.left but cannot even access c.left
                            node.left = c.left;
                        }
                        else if c.left.is_none() && c.right.is_some() {
                            node.left = c.right;
                        }
                        else {
                            // TODO: walk through child tree and get rightmost element
                        }
                        return true;
                    } else {
                        node = c;
                    }
                }
                else {
                    return false;
                }
            }
        }
    }
}

struct Tree<T> {
    root: Option<Node<T>>
}

impl<T: PartialOrd + Display> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, val: T) {
        match self.root {
            Some(ref mut n) => n.insert(val),
            None => self.root = Some(Node::new(val))
        }
    }

    fn print(&self) {
        match self.root {
            Some(ref n) => n.print_inorder(),
            None => println!("[]")
        }
    }

    fn delete(&mut self, val: T) {
        if let Some(ref mut n) = self.root {
            n.delete(val);
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