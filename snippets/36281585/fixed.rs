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
        let mut to_add = HashMap::new();
        for (key, value) in self.nodes.iter() {
            if self.edges.contains_key(key) {
                for (target, weight) in self.edges[key].iter() {
                    to_add.insert(*target, *weight);
                }
            }
        }
        for (target, weight) in to_add.iter() {
            self.nodes.entry(*target).or_insert(Node::new()).weight;
        }
    }
}

fn main() {}