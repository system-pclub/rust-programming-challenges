use std::cmp::Ordering;

pub struct BinarySearchTree {
    root: OptNode,
    size: u32,
}

type OptNode = Option<Box<Node>>;

struct Node {
    key: i32,
    left: OptNode,
    right: OptNode,
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn contains(&self, key: i32) -> bool {
        let mut node = &self.root;
        while let Some(ref boxed_node) = *node {
            match key.cmp(&boxed_node.key) {
                Ordering::Less => node = &boxed_node.left,
                Ordering::Greater => node = &boxed_node.right,
                Ordering::Equal => return true,
            }
        }

        false
    }

    pub fn insert(&mut self, key: i32) {
        let mut node = &mut self.root;
        while let Some(ref mut boxed_node) = *node {
            match key.cmp(&boxed_node.key) {
                Ordering::Less => node = &mut boxed_node.left,
                Ordering::Greater => node = &mut boxed_node.right,
                Ordering::Equal => break,
            }
        }
        if node.is_none() {
            *node = Some(Box::new(Node {
                key: key,
                left: None,
                right: None,
            }));
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let mut bst = BinarySearchTree::new();
        assert!(!bst.contains(1));
        bst.insert(1);
        assert!(bst.contains(1));
    }
}