use std::rc::Rc;

struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    data: String,
}


impl Node {
    fn new(_data: String) -> Node {
        Node {
            data : _data.clone(),
            left : None,
            right : None,
        }
    }

    fn insert_left(&mut self, mut node: &Rc<Node>) {
        self.left = Some(node.clone());
    }

    fn insert_right(&mut self, mut node: &Rc<Node>) {
        self.left = Some(node.clone());
    }
}

fn set_root_and_left(root: &str, left: &str) -> (String, String) {
    let mut root = Rc::new(Node::new(String::from(root)));
    let mut left = Rc::new(Node::new(String::from(left)));
    root.insert_left(&left);
    let x = (root.data.clone(), root.left.as_ref().unwrap().data.clone());
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