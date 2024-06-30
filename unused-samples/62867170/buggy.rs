use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

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

fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match (t1, t2) {
        (Some(t1_), Some(t2_)) => {
            let mut t1 = t1_.borrow_mut();
            let mut t2 = t2_.borrow_mut();
            t1.val += t2.val;
            t1.left = merge_trees(t1.left.take(), t2.left.take());
            t1.right = merge_trees(t1.right.take(), t2.right.take());
            Some(t1_)
        }
        (Some(t1), None) => Some(t1),
        (None, Some(t2)) => Some(t2),
        (None, None) => None,
    }
}

fn main() {
    let _t1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let _t2 = Rc::new(RefCell::new(TreeNode::new(2)));

    let mut head = TreeNode::new(_t1.borrow().val);
    head.left = merge_trees(_t1.borrow().left, _t2.borrow().left);
}