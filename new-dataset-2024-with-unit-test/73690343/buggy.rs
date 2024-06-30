use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn invalid_path_error(self) {
        panic!("Invalid path");
    }

    pub fn insert(&mut self, directions: &[&str], val: i32) {
        let mut cur_node = &mut None;

        let l = directions.len();

        if directions[0] == "left" {
            cur_node = &mut self.left;
        }
        if directions[0] == "right" {
            cur_node = &mut self.right;
        }

        for dir in &directions[1..] {
            let mut n;

            if *dir == "left" {
                if let Some(z) = cur_node {
                    n = &mut z.borrow_mut().left;
                } else {
                    panic!("Invalid path");
                }
            } else if *dir == "right" {
                if let Some(z) = cur_node {
                    n = &mut z.borrow_mut().right;
                } else {
                    panic!("Invalid path");
                }
            } else {
                panic!("invalid direction!");
            }

            cur_node = n;
        }
    }
}

fn task(values: &[i32; 4]) -> String {
    let mut tree = TreeNode::new(0);
    tree.insert(&["left"], values[0]);
    tree.insert(&["right"], values[1]);
    tree.insert(&["left", "left"], values[2]);
    tree.insert(&["right", "left"], values[3]);
    format!(
        "{} {} {} {}",
        tree.val,
        tree.left.unwrap().borrow().val,
        tree.right.unwrap().borrow().val,
        tree.left
            .unwrap()
            .borrow()
            .left
            .unwrap()
            .borrow()
            .val
    )
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let values = [1, 2, 2, 3];
        assert_eq!(task(&values), "0 1 2 2");
    }
}
