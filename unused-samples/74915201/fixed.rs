use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct TreeNode {
    left: Option<Rc<TreeNode>>,
    right: Option<Rc<TreeNode>>,
    sibling: RefCell<Option<Weak<TreeNode>>>,
    v: u8,
}

impl TreeNode {
    pub fn new(v: u8) -> Rc<Self> {
        Rc::new(TreeNode {
            v,
            ..TreeNode::default()
        })
    }
    pub fn new_with(left: Option<Rc<TreeNode>>, right: Option<Rc<TreeNode>>, v: u8) -> Rc<Self> {
        Rc::new(TreeNode {
            left,
            right,
            v,
            sibling: RefCell::new(None),
        })
    }
    pub fn set_siblings(self: &Rc<Self>) {
        let Some(left) = self.left() else { return };
        let right = self.right();

        left.sibling.replace(right.map(Rc::downgrade));

        if let Some(sibling) = self.sibling() {
            // not sure this is correct, depending on construction, with
            // 3  5
            //  \  \
            //  2  4
            //   \/
            //   1
            // (2) has a sibling (4) but doesn't have a right node, so
            // unconditionally setting right.sibling doesn't seem correct
            right
                .unwrap()
                .sibling
                .replace(sibling.left().map(Rc::downgrade));
        }

        left.set_siblings();
        right.map(|r| r.set_siblings());
    }

    pub fn left(&self) -> Option<&Rc<Self>> {
        self.left.as_ref()
    }
    pub fn right(&self) -> Option<&Rc<Self>> {
        self.right.as_ref()
    }
    pub fn sibling(&self) -> Option<Rc<Self>> {
        self.sibling.borrow().as_ref()?.upgrade()
    }
}

fn task(values: [u8; 5]) -> Rc<TreeNode> {
    let t = TreeNode::new_with(
        TreeNode::new_with(
            TreeNode::new(values[0]).into(),
            TreeNode::new(values[1]).into(),
            values[2],
        )
        .into(),
        TreeNode::new(values[3]).into(),
        values[4],
    );
    t.set_siblings();
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let t = task([1, 2, 3, 4, 5]);
        assert_eq!(t.left().and_then(|l| l.sibling()).unwrap().v, 4);
        let ll = t.left().and_then(|l| l.left());
        assert_eq!(ll.map(|ll| ll.v), Some(1));
        ll.unwrap().sibling().unwrap();
        assert_eq!(
            t.left()
                .and_then(|l| l.left())
                .and_then(|ll| ll.sibling())
                .unwrap()
                .v,
            2
        );
    }
}
