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
                Some(ref mut box_node) if box_node.key != key => {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mut() {
        let mut tree = BinaryTree {
            root: Some(Box::new(Node {
                key: 5,
                value: 5,
                left: Some(Box::new(Node {
                    key: 3,
                    value: 3,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    key: 8,
                    value: 8,
                    left: None,
                    right: None,
                })),
            })),
        };

        let node = tree.find_mut(3);
        assert_eq!(node.unwrap().value, 3);
        let node = tree.find_mut(5);
        assert_eq!(node.unwrap().value, 5);
        let node = tree.find_mut(8);
        assert_eq!(node.unwrap().value, 8);
        let node = tree.find_mut(10);
        assert!(node.is_none());
    }
}