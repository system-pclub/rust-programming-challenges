pub struct Node{
    pub children: Vec<Box<Node>>,
    pub parent: Option<Box<Node>>,
    pub value: f32,
    //.....
}

pub mod tree {
    use crate::Node;
    pub fn build_node(parent: Option<Box<Node>>)-> Node{
        Node{
            children: vec![],
            parent,
            value: 0.0,
    
        }
    }
}

fn main() {
    let mut root_nd = tree::build_node(None);
    let mut next_nd = tree::build_node(Some(Box::new(root_nd)));
    root_nd.children.push(Box::new(next_nd));
}