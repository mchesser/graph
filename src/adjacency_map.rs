use Graph;

use std::ops::Index;
use std::collections::HashMap;
use std::collections::hash_map::Keys;

pub type NodeId = usize;
type ListId = usize;

pub struct AdjacencyMapNode<V, W> {
    pub value: V,
    pub outgoing: HashMap<NodeId, W>,
}

pub struct AdjacencyMap<V, W> {
    nodes: Vec<AdjacencyMapNode<V, W>>,
    map: HashMap<NodeId, ListId>,
}

impl<V, W> AdjacencyMap<V, W> {
    pub fn new() -> AdjacencyMap<V, W> {
        AdjacencyMap {
            nodes: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_id: NodeId, value: V) {
        let list_id = self.nodes.len();
        self.nodes.push(AdjacencyMapNode {
            value: value,
            outgoing: HashMap::new(),
        });
        self.map.insert(node_id, list_id);
    }

    pub fn add_arc(&mut self, from: NodeId, to: NodeId, weight: W) {
        self.nodes[self.map[from]].outgoing.insert(to, weight);
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId, weight: W) where W: Clone {
        self.add_arc(a, b, weight.clone());
        self.add_arc(b, a, weight.clone());
    }
}

impl<V, W> Index<NodeId> for AdjacencyMap<V, W> {
    type Output = AdjacencyMapNode<V, W>;

    fn index(&self, index: &NodeId) -> &AdjacencyMapNode<V, W> {
        &self.nodes[self.map[*index]]
    }
}

// Currently there is no easy way to return an iterator that uses a closure, so we manually map the
// inner iterator instead
struct OutgoingEdgesIter<'a, W: 'a> {
    from: NodeId,
    iter_base: Keys<'a, usize, W>,
}

impl<'a, W> Iterator for OutgoingEdgesIter<'a, W> {
    type Item = (NodeId, NodeId);
    fn next(&mut self) -> Option<(NodeId, NodeId)> {
        match self.iter_base.next() {
            Some(&to) => Some((self.from, to)),
            None => None,
        }
    }
}

impl<'a, N, W> Graph for &'a AdjacencyMap<N, W> where W: Clone {
    type NodeId = NodeId;
    type Edge = (NodeId, NodeId);
    type Weight = W;
    type OutgoingEdgesIter = OutgoingEdgesIter<'a, W>;

    fn target(&self, edge: &(NodeId, NodeId)) -> Option<NodeId> {
        if self.map.contains_key(&edge.1) { Some(edge.1) }
        else { None }
    }

    fn weight(&self, edge: &(NodeId, NodeId)) -> W {
        let &(from, to) = edge;
        let node = self.map.get(&from).expect("Edge does not exist");
        self.nodes[*node].outgoing.get(&to).cloned().expect("Target node does not exist")
    }

    fn outgoing_edges(&self, node: &NodeId) -> OutgoingEdgesIter<'a, W> {
        OutgoingEdgesIter {
            from: *node,
            iter_base: self.nodes[self.map[*node]].outgoing.keys(),
        }
    }
}

