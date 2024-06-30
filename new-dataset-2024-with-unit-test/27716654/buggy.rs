use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Id {
    id: i32,  // let’s pretend it’s that
}

struct Node {
    children: HashMap<Id, Box<Node>>,
    decoration: String,
    // other fields
}

struct Tree {
   root: Box<Node>
}

impl Tree {
    /// Traverse the nodes along the specified path.
    /// Return the node at which traversal stops either because the path is exhausted
    /// or because there are no more nodes matching the path.
    /// Also return any remaining steps in the path that did not have matching nodes.
    fn traverse_path<'p>(&mut self, mut path: &'p [Id]) -> (&mut Box<Node>, &'p [Id]) {
        let mut node = &mut self.root;
        loop {
            match node.children.get_mut(&path[0]) {
                Some(child_node) => {
                    path = &path[1..];
                    node = child_node;
                },
                None => {
                    break;
                }
            }
        }
        (node, path)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tree = Tree {
            root: Box::new(Node {
                children: HashMap::new(),
                decoration: "root".to_string(),
            }),
        };
        let root = tree.traverse_path(&[Id { id: 1 }]).0;
        println!("{:?}", root.decoration); // root
        assert_eq!(root.decoration, "root");
        root.children.insert(Id { id: 2 }, Box::new(Node {
            children: HashMap::new(),
            decoration: "child".to_string(),
        }));
        let (child, rest_path) = tree.traverse_path(&[Id { id: 2 }, Id { id: 3 }]);
        assert_eq!(child.decoration, "child");
        assert_eq!(rest_path.len(), 1);
        assert_eq!(rest_path[0].id, 3);
    }
}