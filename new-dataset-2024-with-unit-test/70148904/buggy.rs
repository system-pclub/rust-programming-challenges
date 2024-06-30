use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

/*
            1
          /  \
         2   3
       / 
      4 
*/

fn make_tree() -> TreeNode {
  let mut t = TreeNode::new(1);
  t.left = Some(Rc::new(RefCell::new (TreeNode::new(2))));
  t.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
  t.left.clone().unwrap().get_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4)))); // throws error
  t
}

fn main() {}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_make_tree() {
      let t = make_tree();
      assert_eq!(t.val, 1);
      assert_eq!(t.left.as_ref().unwrap().borrow().val, 2);
      assert_eq!(t.right.as_ref().unwrap().borrow().val, 3);
      assert_eq!(t.left.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val, 4);
  }
}