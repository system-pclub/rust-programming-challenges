use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    key: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
struct Tree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: Tree<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            key: value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

impl<T: Ord> Tree<T> {
    fn add(&mut self, value: T) {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.key.cmp(&value) {
                Ordering::Less => current = &mut node.right,
                Ordering::Greater => current = &mut node.left,
                Ordering::Equal => current = &mut node.right,
            }
        }

        current.0 = Some(Box::new(Node::new(value)));
    }

    fn successor(&self, value: &T) -> Option<&T> {
        let mut current = self.0.as_ref();
        let mut successor = None;
        while current.is_some() {
            let node = current.unwrap();
            if *value < node.key {
                successor = current;
                current = node.left.0.as_ref();
            } else {
                current = node.right.0.as_ref();
            }
        }

        successor.map(|node| &node.key)
    }

    fn extract_min(&mut self) -> Option<T> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &mut current.0.as_mut().unwrap().left;
            }

            let temp = current.0.take().unwrap();
            node = Some(temp.key);
            current.0 = temp.right.0;
        }

        node
    }

    fn extract_max(&mut self) -> Option<T> {
        let mut node = None;

        if self.0.is_some() {
            let mut current = self;

            while current.0.as_ref().unwrap().right.0.is_some() {
                current = &mut current.0.as_mut().unwrap().right;
            }

            let temp = current.0.take().unwrap();
            node = Some(temp.key);
            current.0 = temp.left.0;
        }

        node
    }
    fn remove(&mut self, value: &T) {
        let mut current = self;

        while let Some(ref mut node) = current.0 {
            match node.key.cmp(value) {
                Ordering::Less => current = &mut current.0.as_mut().unwrap().right,
                Ordering::Greater => current = &mut current.0.as_mut().unwrap().left,
                Ordering::Equal => match (node.left.0.as_mut(), node.right.0.as_mut()) {
                    (None, None) => current.0 = None,
                    (Some(_), None) => current.0 = node.left.0.take(),
                    (None, Some(_)) => current.0 = node.right.0.take(),
                    (Some(_), Some(_)) => {
                        current.0.as_mut().unwrap().key = node.right.extract_min().unwrap();
                    }
                },
            }
        }
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: Tree(None) }
    }
    pub fn add(&mut self, value: T) {
        self.root.add(value);
    }
    pub fn remove(&mut self, value: &T) {
        self.root.remove(value);
    }
    pub fn successor(&self, value: &T) -> Option<&T> {
        self.root.successor(value)
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BinarySearchTree::new();
        bst.add(5);
        bst.add(3);
        bst.add(8);
        bst.add(2);
        bst.add(4);
        bst.add(7);
        bst.add(9);
        bst.add(1);
        bst.add(6);

        assert_eq!(bst.successor(&5), Some(&6));
        assert_eq!(bst.successor(&3), Some(&4));
        assert_eq!(bst.successor(&8), Some(&9));
        assert_eq!(bst.successor(&2), Some(&3));
        assert_eq!(bst.successor(&4), Some(&5));
        assert_eq!(bst.successor(&7), Some(&8));
        assert_eq!(bst.successor(&9), None);
        assert_eq!(bst.successor(&1), Some(&2));
        assert_eq!(bst.successor(&6), Some(&7));

        bst.remove(&5);
        assert_eq!(bst.successor(&4), Some(&6));
    }
}