#[derive(Debug, Clone)]
struct Dir {
    name: String,
    children: Vec<Node>,
}

#[derive(Debug, Clone)]
enum Node {
    File(String),
    Dir(Dir),
}

#[derive(Debug, Clone)]
struct FileSystem {
    root: Dir,
}

// I wanted to flatten this structure into a Vec<&mut Node> (the order is not relevant). 
// My idea was to recursively traverse the tree and add every nodes to a vec:
impl Node {
    fn flatten(&mut self, nodes: &mut Vec<&mut Node>) {
        nodes.push(self);

        if let Node::Dir(dir) = self {
            for node in dir.children.iter_mut() {
                node.flatten(nodes);
            }
        }
    }
}

fn main() {}