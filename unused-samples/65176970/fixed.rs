use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

pub enum Colour {
    Red,
    Black,
}

type Child<T> = Option<Rc<RefCell<Node<T>>>>;
type Parent<T> = Option<Weak<RefCell<Node<T>>>>;

pub struct RBTree<T: Ord> {
    root: Child<T>,
}

impl<T: Ord + Display> RBTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        fn insert<T: Ord>(child: &mut Child<T>, mut new_node: Node<T>) {
            let child = child.as_ref().unwrap();
            let mut child_mut_borrow = child.borrow_mut();

            if child_mut_borrow.value == new_node.value {
                return;
            }

            let leaf = if child_mut_borrow.value > new_node.value {
                &mut child_mut_borrow.left
            } else {
                &mut child_mut_borrow.right
            };

            match leaf {
                Some(_) => {
                    insert(leaf, new_node);
                }
                None => {
                    new_node.parent = Some(Rc::downgrade(&child));
                    *leaf = Some(Rc::new(RefCell::new(new_node)));
                }
            };
        }

        let mut new_node = Node::new(value);

        if self.root.is_none() {
            new_node.parent = None;
            self.root = Some(Rc::new(RefCell::new(new_node)));
        } else {
            // We ensure that a `None` is never sent to insert()
            insert(&mut self.root, new_node);
        }
    }

    pub fn print_tree(&self) -> String {
        fn print_tree<T: Ord + Display>(node: &Child<T>) -> String {
            if let Some(node) = node {
                let node = node.borrow();
                format!(
                    "{} {} {}",
                    print_tree(&node.left),
                    node.value,
                    print_tree(&node.right)
                )
            } else {
                "".to_string()
            }
        }
        print_tree(&self.root)
    }
}

struct Node<T: Ord> {
    value: T,
    colour: Colour,
    parent: Parent<T>,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: value,
            colour: Colour::Red,
            parent: None,
            left: None,
            right: None,
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