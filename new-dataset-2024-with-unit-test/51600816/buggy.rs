use std::cmp::PartialOrd;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: PartialOrd> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

struct Tree<T: PartialOrd> {
    root: Link<T>,
}

impl<T: PartialOrd> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn add(&mut self, val: T) {
        let mut prev: Option<&mut Node<T>> = None;
        let mut cur = self.root.as_mut();

        while let Some(boxed_node_ref) = cur {
            if val < boxed_node_ref.val {
                cur = boxed_node_ref.left.as_mut();
            } else {
                cur = boxed_node_ref.right.as_mut();
            }
            prev = Some(boxed_node_ref);
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        tree.add(5);
        tree.add(3);
        tree.add(8);
        assert_eq!(tree.root.as_ref().unwrap().val, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().val, 3);
        assert_eq!(tree.root.as_ref().unwrap().right.as_ref().unwrap().val, 8);
    }
}