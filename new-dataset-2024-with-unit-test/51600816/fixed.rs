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
        let mut cur = &mut self.root;

        while let Some(ref mut boxed_node_ref) = cur {
            if val < boxed_node_ref.val {
                cur = &mut boxed_node_ref.left;
            } else {
                cur = &mut boxed_node_ref.right;
            }
        }

        let new_node = Box::new(Node {
            val,
            left: None,
            right: None,
        });

        *cur = Some(new_node);
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