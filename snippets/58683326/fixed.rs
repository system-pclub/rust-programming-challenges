use std::rc::Rc;
use std::cell::RefCell;

pub struct Node{
    pub children: Vec<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub value: f32,
    //.....
}

pub mod tree {
    use crate::Node;
    use std::rc::Rc;
    use std::cell::RefCell;
    pub fn build_node(parent: Option<Rc<RefCell<Node>>>)-> Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node{
            children: vec![],
            parent,
            value: 0.0,
    
        }))
    }
}

fn main() {
    let mut root_nd = tree::build_node(None);
    let mut next_nd = tree::build_node(Some(Rc::clone(&root_nd)));
    root_nd.borrow_mut().children.push(Rc::clone(&next_nd));
}