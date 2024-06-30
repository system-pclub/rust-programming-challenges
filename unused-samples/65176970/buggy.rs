#[derive(Debug, Clone, PartialEq)]
pub enum Colour {
    Red,
    Black,
}

type T_Node<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: Copy + Clone + Ord> {
    value: T,
    colour: Colour,
    parent: T_Node<T>,
    left: T_Node<T>,
    right: T_Node<T>,
}

impl<T: Copy + Clone + Ord + std::fmt::Display> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            colour: Colour::Red, // add a new node as red, then fix violations
            parent: None,
            left: None,
            right: None,
            // height: 1,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.value == value {
            return;
        }

        let mut leaf = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        match leaf {
            None => {
                let mut new_node = Node::new(value);
                new_node.parent = Some(Box::new(self));
                new_node.colour = Colour::Red;

                (*leaf) = Some(Box::new(new_node));
            }
            Some(ref mut leaf) => {
                leaf.insert(value);
            }
        };
    }

    pub fn print_tree(&self) -> String {
        let mut result = String::new();

        if let Some(ref left) = self.left {
            result.push_str(&left.print_tree());
        }

        result.push_str(&format!(" {} ", self.value));

        if let Some(ref right) = self.right {
            result.push_str(&right.print_tree());
        }

        result
    }
}

pub struct RBTree<T: Copy + Clone + Ord> {
    root: T_Node<T>,
}

impl<T: Copy + Clone + Ord + std::fmt::Display> RBTree<T> {
    pub fn new() -> RBTree<T> {
        RBTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            root.insert(value);
        } else {
            let mut new_node = Node::new(value);
            new_node.colour = Colour::Black;
            self.root = Some(Box::new(new_node));
        }
    }

    pub fn print_tree(&self) -> String {
        if let Some(ref root) = self.root {
            root.print_tree()
        } else {
            "".to_string()
        }
    }
}

fn main() {
    let mut rbtree = RBTree::<i32>::new();
    rbtree.insert(55);
    rbtree.insert(5);
    rbtree.insert(27);
    rbtree.insert(100);
    rbtree.insert(23);
    assert_eq!(rbtree.print_tree(), " 5  23  27  55  100 ".to_string());
}
