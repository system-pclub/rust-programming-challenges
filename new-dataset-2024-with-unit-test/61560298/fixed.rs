struct Node {
    value: bool,
    next: Option<Box<Node>>,
}

fn populate(next: &mut Option<Box<Node>>) -> Option<&mut Node> {
    let node = Node {
        value: true,
        next: None,
    };
    *next = Some(Box::new(node));
    next.as_deref_mut()
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate() {
        let mut node = Node {
            value: false,
            next: None,
        };
        let result = populate(&mut node.next);
        assert_eq!(node.value, false);
        assert_eq!(result.unwrap().value, true);
    }
}