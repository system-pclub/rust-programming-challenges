use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn task(branch_value: i32, leaf_value: i32) -> Rc<Node> {
    let branch = Rc::new(Node {
        value: branch_value,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let new_leaf = Rc::new(Node {
        value: leaf_value,
        parent: RefCell::new(Rc::downgrade(&branch)),
        children: RefCell::new(vec![]),
    });

    branch.children.borrow_mut().push(new_leaf);
    return branch;
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let branch = task(5, 4);
        assert_eq!(branch.value, 5);
        assert_eq!(branch.children.borrow()[0].value, 4);
        assert_eq!(
            branch.children.borrow()[0]
                .parent
                .borrow()
                .upgrade()
                .unwrap()
                .value,
            5
        );
    }
}
