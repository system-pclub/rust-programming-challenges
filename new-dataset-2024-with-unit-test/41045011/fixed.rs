use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    data: String,
}

impl Node {
    fn new(data: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            left: None,
            right: None,
        }))
    }

    fn insert_left(&mut self, node: Rc<RefCell<Node>>) {
        self.left = Some(node);
    }

    fn insert_right(&mut self, node: Rc<RefCell<Node>>) {
        self.right = Some(node);
    }
}

fn set_root_and_left(root: &str, left: &str) -> (String, String) {
    let mut root = Node::new(String::from("root"));
    let mut left = Node::new(String::from("left"));
    root.borrow_mut().insert_left(left);
    let x = (root.borrow().data.clone(), root.borrow().left.as_ref().unwrap().borrow().data.clone());
    x
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_root_and_left() {
        let (root, left) = set_root_and_left("root", "left");
        assert_eq!(root, "root");
        assert_eq!(left, "left");
    }
}