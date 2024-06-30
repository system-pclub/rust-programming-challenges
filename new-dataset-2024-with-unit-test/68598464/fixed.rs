use std::cell::RefCell;
use std::rc::Rc;

pub struct WeightedDirectedGraph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

pub struct Node {
    edges: Vec<Edge>,
}

pub struct Edge {
    to_node: Rc<RefCell<Node>>,
    cost: f32,
}

impl WeightedDirectedGraph {
    pub fn new() -> Self {
        WeightedDirectedGraph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node { edges: Vec::new() }));
        self.nodes.push(node.clone());
        node
    }

    pub fn add_edge(&mut self, from_node_index: usize, to_node_index: usize, cost: f32) {
        let from_node = self.nodes[from_node_index].clone();
        let to_node = self.nodes[to_node_index].clone();
        let new_edge = Edge { to_node, cost };
        from_node.borrow_mut().edges.push(new_edge);
    }

    pub fn total_cost(&self) -> f32 {
        self.nodes.iter().map(|node| {
            node.borrow().edges.iter().map(|edge| edge.cost).sum::<f32>()
        }).sum()
    }

}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_directed_graph() {
        let mut graph = WeightedDirectedGraph::new();

        let node1 = graph.add_node();
        let node2 = graph.add_node();
        let node3 = graph.add_node();

        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 2.0);
        graph.add_edge(2, 0, 3.0);

        assert_eq!(graph.total_cost(), 6.0);
    }
}