pub struct WeightedDirectedGraph<'a> {
    nodes: Vec<Node<'a>>,
}

pub struct Node<'a> {
    edges: Vec<Edge<'a>>,
}

pub struct Edge<'a> {
    to_node: &'a Node<'a>,
    cost: &'a f32,
}

impl<'a> WeightedDirectedGraph<'a> {
    pub fn new() -> Self {
        WeightedDirectedGraph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self) -> &Node<'a> {
        let new_node: Node<'a> = Node { edges: Vec::new() };
        self.nodes.push(new_node);
        &self.nodes[self.nodes.len() - 1]
    }

    pub fn add_edge(&mut self, from_node_index: usize, to_node_index: usize, cost: f32) {
        // get the index before adding it, since index is zero-based
        let new_edge_index = self.nodes[from_node_index].edges.len();
        let mut new_edge: Edge<'a> = Edge {
            to_node: &self.nodes[to_node_index],
            cost: &cost,
        };
    }

    pub fn total_cost(&self) -> f32 {
        self.nodes.iter().map(|node| {
            node.edges.iter().map(|edge| *edge.cost).sum::<f32>()
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