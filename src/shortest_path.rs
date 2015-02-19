use Graph;

use std::ops::Add;
use std::cmp::{PartialOrd, Ordering};
use std::iter::IteratorExt;
use std::hash::Hash;

use std::collections::hash_map::{HashMap, Hasher, Entry};
use std::collections::BinaryHeap;

use num;
use num::traits::Zero;

pub type Heuristic<N, W> = fn(from: &N, to: &N) -> W;

pub fn a_star<G: Graph>(graph: &G, start: G::NodeId, end: G::NodeId,
    heuristic: Heuristic<G::NodeId, G::Weight>) -> Option<Vec<G::NodeId>>
    where G::NodeId: Hash<Hasher> + Eq + Clone,
          G::Neighbours: Iterator<Item=G::NodeId>,
          G::Weight: Clone + Ord + PartialOrd + Add<Output=G::Weight> + Zero
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
                    Some(v) => dist_map[v].clone(),
                    None => return Some(path),
                }
            }
        }

        // Insert the current value if it is better than any previous value inserted
        match dist_map.entry(active_val.node.clone()) {
            Entry::Occupied(mut e) => {
                if active_val.f_cost() < e.get().f_cost() { e.insert(active_val.clone()); }
                else { continue }
            },
            Entry::Vacant(e) => { e.insert(active_val.clone()); },
        }

        // Visit all the neighbours of the current node that have not already been added to the
        // distance map
        for node in graph.neighbours(&active_val.node).filter(|n| !dist_map.contains_key(n)) {
            // Get the cost of moving from the current node to the next node
            let move_cost = match graph.weight(&active_val.node, &node) {
                Some(c) => c,
                None => break,
            };

            let mut next_val = PathNode::new(node, Some(active_val.node.clone()));
            next_val.g_cost = active_val.g_cost.clone() + move_cost;
            next_val.h_cost = heuristic(&next_val.node, &end);
            frontier.push(next_val);
        }
    }

    // No path to the end node could be found
    None
}

#[derive(Clone)]
struct PathNode<N, W> {
    node: N,
    parent: Option<N>,
    h_cost: W,
    g_cost: W,
}

impl<N, W: Zero + Add> PathNode<N, W> {
    pub fn new(node: N, parent: Option<N>) -> PathNode<N, W> {
        PathNode {
            node: node,
            h_cost: num::zero(),
            g_cost: num::zero(),
            parent: parent,
        }
    }
}

impl<N, W> PathNode<N, W> where W: Add<Output=W> + Clone {
    pub fn f_cost(&self) -> W {
        self.h_cost.clone() + self.g_cost.clone()
    }
}

impl<N, W> PartialEq for PathNode<N, W> where W: PartialEq + Add<Output=W> + Clone {
    fn eq(&self, other: &PathNode<N, W>) -> bool {
        self.f_cost() == other.f_cost()
    }
}
impl<N, W> Eq for PathNode<N, W> where W: PartialEq + Add<Output=W> + Clone {}

impl<N, W> PartialOrd for PathNode<N, W> where W: PartialOrd + Add<Output=W> + Clone {
    fn partial_cmp(&self, other: &PathNode<N, W>) -> Option<Ordering> {
        // Reverse the ordering so that it makes a min queue
        other.f_cost().partial_cmp(&self.f_cost())
    }
}
impl<N, W> Ord for PathNode<N, W> where W: Ord + Add<Output=W> + Clone {
    fn cmp(&self, other: &PathNode<N, W>) -> Ordering {
        // Reverse the ordering so that it makes a min queue
        other.f_cost().cmp(&self.f_cost())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use AdjacencyMap;

    fn no_heuristic(_: &usize, _: &usize) -> u32 { 0 }

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
