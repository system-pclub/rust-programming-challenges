#[derive(Debug)]
struct Data {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Node {
    value: Data,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct SortedContainer {
    root: Option<Box<Node>>,
}

impl SortedContainer {
    pub fn insert(&mut self, name: String, age: i32) {
        let d = Data {
            name: name,
            age: age,
        };
        let n = Node {
            value: d,
            left: None,
            right: None,
        };
        match self.root {
            Some(ref mut rootnode) => SortedContainer::node_insert(n, rootnode),
            None => self.root = Some(Box::new(n)),
        }
    }

    fn node_insert(n: Node, n2: &mut Node) {
        let i = SortedContainer::data_compare(&n.value, &n2.value);
        if i < 0 {
            match n2.right {
                Some(ref mut rightnode) => SortedContainer::node_insert(n, rightnode),
                None => n2.right = Some(Box::new(n)),
            }
        } else if i > 0 {
            match n2.left {
                Some(ref mut leftnode) => SortedContainer::node_insert(n, leftnode),
                None => n2.left = Some(Box::new(n)),
            }
        }
    }

    fn data_compare(d: &Data, d2: &Data) -> i32 {
        if d.age < d2.age {
            return -1;
        } else if d.age > d2.age {
            return 1;
        } else if d.name == d2.name {
            return 0;
        } else if d.name > d2.name {
            return 1;
        } else if d.name < d2.name {
            return -1;
        } else {
            return 0;
        }
    }

    fn find_age(&self, name: &str) -> Option<i32> {
        let mut current = &self.root;
        fn find_age_inner(node: &Option<Box<Node>>, name: &str) -> Option<i32> {
            match node {
                Some(n) => {
                    if n.value.name == name {
                        return Some(n.value.age);
                    }
                    if let Some(age) = find_age_inner(&n.left, name) {
                        return Some(age);
                    }
                    if let Some(age) = find_age_inner(&n.right, name) {
                        return Some(age);
                    }
                    return None;
                }
                None => None,
            }
        }
        find_age_inner(current, name)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut sc = SortedContainer { root: None };
        sc.insert("Alice".to_string(), 20);
        sc.insert("Bob".to_string(), 25);
        sc.insert("Charlie".to_string(), 30);
        sc.insert("David".to_string(), 35);
        sc.insert("Eve".to_string(), 40);
        sc.insert("Frank".to_string(), 45);
        assert_eq!(sc.find_age("Alice").unwrap(), 20);
        assert_eq!(sc.find_age("Bob").unwrap(), 25);
        assert_eq!(sc.find_age("Charlie").unwrap(), 30);
        assert_eq!(sc.find_age("David").unwrap(), 35);
        assert_eq!(sc.find_age("Eve").unwrap(), 40);
        assert_eq!(sc.find_age("Frank").unwrap(), 45);
        assert!(sc.find_age("George").is_none());
    }
}
