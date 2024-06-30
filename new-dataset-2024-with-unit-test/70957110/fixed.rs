use std::{cell::RefCell, cmp::Ordering, fmt::Debug, rc::Rc};

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
        //if no root, just create one
        let mut node = if let Some(root) = &self.root {
            Rc::clone(root)
        } else {
            self.root = Some(Rc::new(RefCell::new(Node {
                value,
                left: None,
                right: None,
            })));
            return true;
        };

        loop {
            let current_node = Rc::clone(&node);
            let mut current_node = RefCell::borrow_mut(&current_node);
            let cmp = current_node.value.cmp(&value);
            let next_node = match cmp {
                Ordering::Less => &mut current_node.left,
                Ordering::Equal => return false,
                Ordering::Greater => &mut current_node.right,
            };
            if let Some(next_node) = next_node {
                node = Rc::clone(next_node);
            } else {
                *next_node = Some(Rc::new(RefCell::new(Node {
                    value,
                    left: None,
                    right: None,
                })));

                return true;
            }
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
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(1), false);
    }
}