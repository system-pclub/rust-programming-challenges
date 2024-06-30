#[derive(Clone)]
pub enum Node {
    Value(u32),
    Branch(u32, Box<Node>, Box<Node>),
}

pub fn zero_node(tree: &Node, node_index: u8) -> Node {
    let mut new_tree = tree.clone();

    fn zero_rec(node: &mut Node, node_count: u8, node_index: u8) -> u8 {
        if node_index == node_count {
            match node {
                &mut Node::Value(_) => {
                    *node = Node::Value(0);
                }
                &mut Node::Branch(_, ref mut left, ref mut right) => {
                    *node = Node::Branch(0, *left, *right);
                }
            }
            node_count
        } else {
            match node {
                &mut Node::Value(val) => 1,
                &mut Node::Branch(_, ref mut left, ref mut right) => {
                    let count_left = zero_rec(&mut **left, node_count + 1, node_index);
                    let count_right = zero_rec(&mut **right, node_count + 1 + count_left, node_index);
                    count_left + count_right + 1
                }
            }
        }
    }

    zero_rec(&mut new_tree, 0, node_index);

    new_tree
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Node::Branch(1, Box::new(Node::Value(2)), Box::new(Node::Value(3)));
        let new_root = zero_node(&root, 2);
        match new_root {
            Node::Branch(val, left, right) => {
                assert_eq!(val, 1);
                match *left {
                    Node::Value(val) => assert_eq!(val, 2),
                    _ => panic!("Expected Node::Value"),
                }
                match *right {
                    Node::Value(val) => assert_eq!(val, 0),
                    _ => panic!("Expected Node::Value"),
                }
            }
            _ => panic!("Expected Node::Branch"),
        }
    }
}
