struct Node {
    key: i32,
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinaryTree {
     root: Option<Box<Node>>,
}

impl BinaryTree {

    fn find_mut(&mut self, key: i32) -> &mut Option<Box<Node>> {
        let mut node = &mut self.root;
        loop {
            match node {
                Some(box_node) if box_node.key != key => {
                    node = if box_node.key < key {
                        &mut box_node.right
                    } else {
                        &mut box_node.left
                    }
                },
                other => return other
            }
        }
    }
}


fn main() {}