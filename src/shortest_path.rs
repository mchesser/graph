use std::{
    cmp::{Ordering, PartialOrd},
    collections::{
        hash_map::{Entry, HashMap},
        BinaryHeap,
    },
    hash::Hash,
    ops::Add,
};

use num_traits::Zero;

use crate::Graph;

pub type Heuristic<N, W> = fn(from: &N, to: &N) -> W;

pub fn a_star<G: Graph>(
    graph: &G,
    start: G::NodeId,
    end: G::NodeId,
    heuristic: Heuristic<G::NodeId, G::Weight>,
) -> Option<Vec<G::NodeId>>
where
    G::NodeId: Hash + Eq + Clone,
    G::Edge: Clone,
    G::Weight: Clone + Ord + PartialOrd + Add + Zero,
{
    let mut dist_map: HashMap<_, PathNode<_, G::Weight>> = HashMap::new();

    let mut frontier = BinaryHeap::new();
    frontier.push(PathNode::new(start.clone(), None));

    while let Some(mut active_val) = frontier.pop() {
        // Check if a complete path has been found
        if active_val.node == end {
            // Follow the path back from the end node to the start node
            let mut path = vec![];
            loop {
                path.push(active_val.node.clone());
                active_val = match active_val.parent.clone() {
                    Some(v) => dist_map[&v].clone(),
                    None => return Some(path),
                }
            }
        }

        // Insert the current value if it is better than any previous value inserted
        match dist_map.entry(active_val.node.clone()) {
            Entry::Occupied(mut e) => {
                if active_val.path_cost < e.get().path_cost {
                    e.insert(active_val.clone());
                }
                else {
                    continue;
                }
            }
            Entry::Vacant(e) => {
                e.insert(active_val.clone());
            }
        }

        // Visit all the neighbours of the current node that have not already been added to the
        // distance map
        for edge in graph.outgoing_edges(&active_val.node) {
            let target = match graph.target(&edge) {
                Some(n) => n,
                None => continue,
            };
            if !dist_map.contains_key(&target) {
                let mut next_val = PathNode::new(target, Some(active_val.node.clone()));
                next_val.path_cost = active_val.path_cost.clone() + graph.weight(&edge);
                next_val.heuristic_cost = heuristic(&next_val.node, &end);
                frontier.push(next_val);
            }
        }
    }

    // No path to the end node could be found
    None
}

#[derive(Clone)]
struct PathNode<N, W> {
    node: N,
    parent: Option<N>,
    path_cost: W,
    heuristic_cost: W,
}

impl<N, W: Zero + Add> PathNode<N, W> {
    pub fn new(node: N, parent: Option<N>) -> PathNode<N, W> {
        PathNode { node, path_cost: num_traits::zero(), heuristic_cost: num_traits::zero(), parent }
    }
}

impl<N, W> PathNode<N, W>
where
    W: Add<Output = W> + Clone,
{
    pub fn total_cost(&self) -> W {
        self.heuristic_cost.clone() + self.path_cost.clone()
    }
}

impl<N, W> PartialEq for PathNode<N, W>
where
    W: PartialEq + Add<Output = W> + Clone,
{
    fn eq(&self, other: &PathNode<N, W>) -> bool {
        self.total_cost() == other.total_cost()
    }
}
impl<N, W> Eq for PathNode<N, W> where W: PartialEq + Add<Output = W> + Clone {}

impl<N, W> PartialOrd for PathNode<N, W>
where
    W: PartialOrd + Add<Output = W> + Clone,
{
    fn partial_cmp(&self, other: &PathNode<N, W>) -> Option<Ordering> {
        // Reverse the ordering so that it makes a min queue
        other.total_cost().partial_cmp(&self.total_cost())
    }
}
impl<N, W> Ord for PathNode<N, W>
where
    W: Ord + Add<Output = W> + Clone,
{
    fn cmp(&self, other: &PathNode<N, W>) -> Ordering {
        // Reverse the ordering so that it makes a min queue
        other.total_cost().cmp(&self.total_cost())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdjacencyMap;

    fn no_heuristic(_: &usize, _: &usize) -> u32 {
        0
    }

    #[test]
    fn test_simple() {
        let mut graph = AdjacencyMap::new();

        graph.add_node(1, ());
        graph.add_node(2, ());
        graph.add_node(3, ());
        graph.add_node(4, ());
        graph.add_node(5, ());
        graph.add_node(6, ());

        graph.add_edge(1, 2, 7);
        graph.add_edge(1, 3, 9);
        graph.add_edge(1, 6, 14);
        graph.add_edge(2, 3, 10);
        graph.add_edge(2, 4, 15);
        graph.add_edge(3, 4, 11);
        graph.add_edge(3, 6, 2);
        graph.add_edge(4, 5, 6);
        graph.add_edge(5, 6, 14);

        let (start, end) = (1, 5);
        let path = a_star(&&graph, start, end, no_heuristic);
        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![5, 6, 3, 1]);
    }

    #[test]
    fn test_simple_no_path() {
        let mut graph = AdjacencyMap::new();

        graph.add_node(1, ());
        graph.add_node(2, ());
        graph.add_node(3, ());

        graph.add_edge(1, 2, 1);
        let (start, end) = (1, 3);
        let path = a_star(&&graph, start, end, no_heuristic);
        assert!(path.is_none());
    }
}
