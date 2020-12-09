use std::collections::HashMap;

struct Node {
    weight:   f64,
    outbound: f64,
}

impl Node {
    fn new() -> Self {
        Node {
            weight: 0.0,
            outbound: 0.0
        }
    }
}

struct Graph {
    edges: HashMap<u32, HashMap<u32, f64>>,
    nodes: HashMap<u32, Node>,
}

impl Graph {
    fn mutate(&mut self) {
        for (key, value) in self.nodes.iter() {
            if self.edges.contains_key(key) {
                for (target, weight) in self.edges[key].iter() {
                    self.nodes.entry(*target).or_insert(Node::new()).weight;
                }
            }
        }
    }
}

fn main() {}