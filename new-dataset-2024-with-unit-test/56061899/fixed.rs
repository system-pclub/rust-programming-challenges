use std::cell::{Ref, RefCell};

pub struct Node<T>
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn find<T>(node: &Box<Node<T>>, data: T) -> Option<&Box<Node<T>>>
where
    T: Copy + PartialOrd + std::fmt::Display,
{
    if node.data == data {
        return Some(node);
    } else if node.data > data {
        if let Some(left) = &node.left {
            let myref = left;
            return find(myref, data);
        } else {
            return None;
        }
    } else {
        if let Some(right) = &node.right {
            let myref = right;
            return find(myref, data);
        } else {
            return None;
        }
    }
}

fn make_tree(x: i32, y: i32, z: i32) -> Box<Node<i32>> {
    Box::new(Node {
        data: x,
        left: Some(Box::new(Node {
            data: y,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            data: z,
            left: None,
            right: None,
        })),
    })
}

fn task(x: i32, y: i32, z: i32) -> i32 {
    let root = make_tree(x, y, z);

    let found = find(&root, x);
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