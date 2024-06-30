#[derive(Debug)]
struct Node<'source> {
    slice: &'source str,
    nid: usize,
    parent: usize,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Tree<'source> {
    source: String,
    nodes: Vec<Node<'source>>,
}

impl<'source> Tree<'source> {
    fn new() -> Self {
        Tree {
            source: String::from("Hello World"),
            nodes: vec![],
        }
    }

    fn slice(&'source mut self, from: usize, to: usize) {
        let new_nid = self.nodes.len();
        self.nodes.push(Node {
            slice: &self.source[from..to],
            nid: new_nid,
            parent: 0,
            children: vec![],
        });
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.slice(2, 6);
    println!("{:?}", tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let mut tree = Tree::new();
        tree.slice(2, 6);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].slice, "llo ");
    }
}
