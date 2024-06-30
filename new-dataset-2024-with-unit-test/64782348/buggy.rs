use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn find_min(mut node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    while node.as_ref().unwrap().borrow().left.is_some() {
        node = &node.as_ref().unwrap().borrow().left;
    }
    node.as_ref().unwrap().borrow().val
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }));
        assert_eq!(find_min(&Some(root.clone())), 2);
    }
}