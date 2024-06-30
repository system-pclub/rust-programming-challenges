use std::borrow::BorrowMut;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let mut leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    leaf.borrow_mut().value = 4;
}

fn task(branch_value: i32, leaf_value: i32) -> Rc<Node> {
    let mut leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: branch_value,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    leaf.borrow_mut().value = leaf_value;
    return branch;
}

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