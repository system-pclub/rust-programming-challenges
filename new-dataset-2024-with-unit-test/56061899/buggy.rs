use std::cell::{Ref, RefCell};

pub struct Node<T>
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    data: T,
    left: Option<RefCell<Box<Node<T>>>>,
    right: Option<RefCell<Box<Node<T>>>>,
}

fn find<T>(node: Ref<Box<Node<T>>>, data: T) -> Option<Ref<Box<Node<T>>>>
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    if node.data == data {
        return Some(node);
    } else if node.data > data {
        if node.left.is_some() {
            // Error: Cannot move out of borrowed content
            let myref = node.left.unwrap().borrow();
            return find(myref, data);
        } else {
            return None;
        }
    } else {
        if node.right.is_some() {
            // Error: returns a value referencing data owned by the current function
            let myref = node.right.as_ref().unwrap().borrow();
            return find(myref, data);
        } else {
            return None;
        }
    }
}

fn make_tree(x: i32, y: i32, z: i32) -> Box<Node<i32>> {
    Box::new(Node {
        data: x,
        left: Some(RefCell::new(Box::new(Node {
            data: y,
            left: None,
            right: None,
        }))),
        right: Some(RefCell::new(Box::new(Node {
            data: z,
            left: None,
            right: None,
        }))),
    })
}


fn task(x: i32, y: i32, z: i32) -> i32 {
    let root = RefCell::new(make_tree(x, y, z));

    let found = find(root.borrow(), x);
    return found.unwrap().data;
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(task(5, 3, 8), 5);
    }
}