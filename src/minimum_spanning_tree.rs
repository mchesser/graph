use std::{
    cmp::{Ordering, PartialOrd},
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use crate::Graph;

pub fn prims<G: Graph>(graph: &G, start: G::NodeId) -> Vec<G::Edge>
where
    G::NodeId: Hash + Eq + Clone,
    G::Edge: Clone,
    G::Weight: Clone + Ord + PartialOrd,
{
    let mut edges = Vec::new();
    let mut connected_nodes = HashSet::new();
    connected_nodes.insert(start.clone());

    let mut active_edges = BinaryHeap::new();
    active_edges.extend(
        graph.outgoing_edges(&start).map(|e| EdgeContainer { cost: graph.weight(&e), edge: e }),
    );

    while let Some(edge) = active_edges.pop() {
        let current_node = match graph.target(&edge.edge) {
            Some(n) => n,
            None => continue,
        };
        if !connected_nodes.insert(current_node.clone()) {
            // If insert returned false, then this node has already been included by some other edge
            continue;
        }
        edges.push(edge.edge.clone());

        // Add this node's edges to the list of active edges
        active_edges.extend(
            graph
                .outgoing_edges(&current_node)
                .map(|e| EdgeContainer { cost: graph.weight(&e), edge: e })
                .filter(|e| match graph.target(&e.edge) {
                    Some(n) => !connected_nodes.contains(&n),
                    None => false,
                }),
        );
    }

    edges
}

struct EdgeContainer<E, W> {
    cost: W,
    edge: E,
}

//
// Boilerplate code for implementing Ord for EdgeContainer, ensuring that it is implemented so that
// elements placed in a binary heap will form a min queue.
//

impl<E, W> PartialEq for EdgeContainer<E, W>
where
    W: Eq,
{
    fn eq(&self, other: &EdgeContainer<E, W>) -> bool {
        self.cost == other.cost
    }
}
impl<E, W> Eq for EdgeContainer<E, W> where W: Eq {}

impl<E, W> PartialOrd for EdgeContainer<E, W>
where
    W: PartialOrd + Eq,
{
    fn partial_cmp(&self, other: &EdgeContainer<E, W>) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}
impl<E, W> Ord for EdgeContainer<E, W>
where
    W: Ord + Clone,
{
    fn cmp(&self, other: &EdgeContainer<E, W>) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[cfg(test)]
mod tests {
    use super::prims;
    use crate::AdjacencyMap;
    use crate::Graph;

    #[test]
    fn test_simple() {
        //  [1] --2-- [2]
        //      \      |
        //        1    2
        //          \  |
        //  [3] --3-- [4]
        let mut graph = AdjacencyMap::new();

        graph.add_node(1, ());
        graph.add_node(2, ());
        graph.add_node(3, ());
        graph.add_node(4, ());

        graph.add_edge(1, 2, 2);
        graph.add_edge(1, 4, 1);
        graph.add_edge(2, 4, 2);
        graph.add_edge(3, 4, 3);

        let mst = prims(&&graph, 1);
        let total = mst.iter().map(|e| (&graph).weight(&e)).fold(0, |acc, x| acc + x);
        assert_eq!(total, 3 + 1 + 2);
    }
}
