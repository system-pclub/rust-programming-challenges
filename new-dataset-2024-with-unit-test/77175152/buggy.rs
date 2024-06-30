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
    #[inline]
    pub fn with_childs(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        let mut pp = p.unwrap();
        let mut qq = q.unwrap();
        if (p.is_none() || q.is_none()) || pp.borrow().val != qq.borrow().val {
            return false;
        }
        return Self::is_same_tree(pp.borrow().left.clone(), qq.borrow().left.clone())
            && Self::is_same_tree(pp.borrow().right.clone(), qq.borrow().right.clone());
    }
}

fn test_create_tree() -> Rc<RefCell<TreeNode>> {
    let left = Rc::new(RefCell::new(TreeNode::new(1)));
    let right = Rc::new(RefCell::new(TreeNode::new(2)));
    let root = Rc::new(RefCell::new(TreeNode::with_childs(
        0,
        Some(left),
        Some(right),
    )));

    root
}

fn test_create_tree2() -> Rc<RefCell<TreeNode>> {
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(2)));
    let root = Rc::new(RefCell::new(TreeNode::with_childs(
        0,
        Some(left),
        Some(right),
    )));

    root
}

fn test_create_tree3() -> Rc<RefCell<TreeNode>> {
    let left = Rc::new(RefCell::new(TreeNode::new(1)));
    let root = Rc::new(RefCell::new(TreeNode::with_childs(0, Some(left), None)));

    root
}

fn main() {}
#[cfg(test)]
mod tests {

    #[test]
    fn test_is_the_same() {
        let t1 = super::test_create_tree();
        let t2 = super::test_create_tree();

        assert!(super::Solution::is_same_tree(Some(t1), Some(t2)));
    }

    #[test]
    fn test_is_not_the_same() {
        let t1 = super::test_create_tree();
        let t2 = super::test_create_tree2();

        assert!(!super::Solution::is_same_tree(Some(t1), Some(t2)));
    }

    #[test]
    fn test_is_the_same_none() {
        let t1 = super::test_create_tree3();
        let t2 = super::test_create_tree3();

        assert!(super::Solution::is_same_tree(Some(t1), Some(t2)));
    }

    #[test]
    fn test_is_not_the_same_none() {
        let t1 = super::test_create_tree();
        let t2 = super::test_create_tree3();

        assert!(!super::Solution::is_same_tree(Some(t1), Some(t2)));
    }
}