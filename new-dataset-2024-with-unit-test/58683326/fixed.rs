use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub value: f32,
    pub name: String,
}

impl Node {
    pub fn new() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            children: vec![],
            parent: None,
            value: 0.0,
            name: "node".to_string(),
        }))
    }

    pub fn from_parent(parent: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            children: vec![],
            parent: Some(parent.clone()),
            value: 0.0,
            name: "node".to_string(),
        }))
    }

    pub fn push_child(&mut self, child: &Rc<RefCell<Node>>) {
        self.children.push(child.clone());
    }
}

fn task(n_children: usize) -> usize {
    let root_nd = Node::new();
    for _ in 0..n_children {
        let next_nd = Node::from_parent(&root_nd);
        root_nd.borrow_mut().children.push(next_nd.clone());
    }
    let x = root_nd.borrow().children.len();
    x
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        assert_eq!(task(3), 3);
        assert_eq!(task(5), 5);
    }
}