pub struct Node{
    pub children: Vec<Box<Node>>,
    pub parent: Option<Box<Node>>,
    pub value: f32,
    pub name: String,
}

impl Node{
    pub fn new()-> Node{
        Node{
            children: vec![],
            parent: None,
            value: 0.0,
            name: "node".to_string(),
        }
    }

    pub fn from_parent(parent: Box<Node>)-> Node{
        Node{
            children: vec![],
            parent: Some(parent),
            value: 0.0,
            name: "node".to_string(),
        }
    }
}

fn task(n_children: usize) -> usize {
    let mut root_nd = Node::new();
    for _ in 0..n_children {
        let next_nd = Node::from_parent(Box::new(root_nd));
        root_nd.children.push(Box::new(next_nd));
    }
    root_nd.children.len()
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