type Link = Option<Box<Node>>;

pub struct Node {
    elem: i32,
    next: Link,
}

fn remove_nth_node_from_end(list: &mut Link, n: usize) {
    if list.is_none() {
        return;
    }
    let get_length = |l: &Link| {
        let mut length = 0;
        let mut current = l;
        while let Some(n) = {current} {
            length += 1;
            current = &n.next;
        }
        length
    };
    let length = get_length(list);
    let mut i = 0;
    let mut current = list;
    while i < length - n {
        if let Some(link) = {current} {
            current = &mut link.next;
        } else {
            panic!("Invalid node!");
        }
        i += 1;
    }
    *current = current.as_mut().unwrap().next.take();
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth() {
        let mut list = Some(Box::new(Node {
            elem: 1,
            next: Some(Box::new(Node {
                elem: 2,
                next: Some(Box::new(Node {
                    elem: 3,
                    next: Some(Box::new(Node {
                        elem: 4,
                        next: Some(Box::new(Node {
                            elem: 5,
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        remove_nth_node_from_end(&mut list, 2);
        let mut current = &list;
        let mut expected = vec![1, 2, 3, 5];
        for &elem in &expected {
            if let Some(node) = {current} {
                assert_eq!(node.elem, elem);
                current = &node.next;
            } else {
                panic!("Invalid node!");
            }
        }
        assert!(current.is_none());
    }
}