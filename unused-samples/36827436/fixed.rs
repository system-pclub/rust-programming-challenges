use std::cmp::Ordering::{Equal, Greater, Less};

type Child = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    val: i32,
    left: Child,
    right: Child,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val: val,
            left: None,
            right: None,
        }
    }

    fn delete(&mut self, val: i32) -> Option<&mut Node> {
        match val.cmp(&self.val) {
            Less => self.left.as_deref_mut().and_then(|n| n.delete(val)),
            Equal => {
                if (self.left.is_none()) && (self.right.is_none()) {
                    None
                } else if self.left.is_none() {
                    self.right.take()
                } else if self.right.is_none() {
                    self.left.take()
                } else {
                    unimplemented!();
                }
            }
            Greater => self.right.as_deref_mut().and_then(|n| n.delete(val)),
        }
    }
}

fn main() {
    let mut node = Node::new(10);
    node.right = Some(Box::new(Node::new(20)));
}
