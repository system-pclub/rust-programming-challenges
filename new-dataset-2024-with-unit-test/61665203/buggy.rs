use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn set_val(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    node.unwrap().get_mut().val = val;
}

fn get_val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    node.unwrap().borrow().val
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_val() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(get_val(&root), 1);
        set_val(&root, 2);
        assert_eq!(get_val(&root), 2);
    }
}
