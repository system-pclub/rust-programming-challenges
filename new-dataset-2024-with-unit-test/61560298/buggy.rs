struct Node {
    value: bool,
    next: Option<Box<Node>>,
}

fn populate(next: &mut Option<Box<Node>>) -> Option<Node> {
    let node = Node {
        value: true,
        next: None,
    };
    let result = Some(Box::new(node));
    *next = result;
    Some(*next.unwrap())
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
