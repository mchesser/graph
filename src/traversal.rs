use Graph;

use std::hash::Hash;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Clone)]
pub struct BfsNode<N> {
    value: N,
    parent: usize,
}

impl<N: Clone> BfsNode<N> {
    pub fn value(&self) -> N {
        self.value.clone()
    }

    pub fn parent(&self) -> usize {
        self.parent
    }
}


pub fn breadth_first_search<F, G: Graph>(graph: &G, start: G::NodeId, mut apply: F)
    where F: FnMut(&[BfsNode<G::NodeId>]) -> bool,
          G::NodeId: Clone + Hash + Eq,
{
    let mut visited = HashSet::new();
    let mut visit_order = vec![];
    let mut frontier = VecDeque::new();

    frontier.push_back(BfsNode { value: start, parent: 0 });
    while let Some(node) = frontier.pop_front() {
        let node_id = visit_order.len();
        visit_order.push(node.clone());

        if !apply(&visit_order) { return }

        for target in graph.outgoing_edges(&node.value).filter_map(|e| graph.target(&e)) {
            if visited.insert(target.clone()) {
                frontier.push_back(BfsNode { value: target, parent: node_id });
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use AdjacencyMap;

    #[test]
    pub fn basic_test() {
        // [1] -> [2] -> [3] -> [4] -> [5]
        let mut graph = AdjacencyMap::new();

        graph.add_node(1, ());
        graph.add_node(2, ());
        graph.add_node(3, ());
        graph.add_node(4, ());
        graph.add_node(5, ());

        graph.add_arc(1, 2, 0);
        graph.add_arc(2, 3, 0);
        graph.add_arc(3, 4, 0);
        graph.add_arc(4, 5, 0);

        let mut index = 1;
        breadth_first_search(&&graph, 1, |visited| {
            assert_eq!(visited.last().map(|n| n.value()), Some(index));
            index += 1;
            true
        });
    }
}
