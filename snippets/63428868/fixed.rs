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
    fn find_mut(&mut self, key: i32) -> Option<&mut Node> {
        // &mut Option<Box<Node>> -> Option<&mut Box<Node>> -> Option<&mut Node>
        let mut node = self.root.as_mut().map(|boxed| boxed.as_mut());
        loop {
            match node {
                Some(box_node) if box_node.key != key => {
                    node = if box_node.key < key {
                        box_node.right.as_mut().map(|boxed| boxed.as_mut())
                    } else {
                        box_node.left.as_mut().map(|boxed| boxed.as_mut())
                    }
                },
                other => return other
            }
        }
    }
}

fn main() {}