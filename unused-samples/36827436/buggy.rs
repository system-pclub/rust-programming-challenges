use std::cmp::Ordering::{Less,Equal,Greater};

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

    fn delete(&mut self, val: i32) {
        //find the node for deletion
        let mut node: &mut Node = self;
        loop {
            match val.cmp(&self.val) {
                Less => {
                    if self.left.is_some() {
                        node = &mut (self.left.as_mut().unwrap());
                    } else {
                        break;
                    }
                }
                Equal => break,
                Greater => node = panic!("nyi"),
            }
        }
        if (node.left.is_none()) && (node.right.is_none()) {
            node = None;
        } else if node.left.is_none() {
            node = node.right.take();
        } else if node.right.is_none() {
            node = node.left.take();
        } else {
            unimplemented!();
        }
    }
}

fn main() {
    let mut node = Node::new(10);
    node.right = Some(Box::new(Node::new(20)));
}