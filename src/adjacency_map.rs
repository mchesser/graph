use Graph;

use std::ops::Index;
use std::collections::HashMap;
use std::iter::Map;
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

type OutgoingIter<'a, W> = Map<Keys<'a, usize, W>, fn(&usize) -> usize>;

impl<'a, N, W> Graph for &'a AdjacencyMap<N, W> where W: Clone {
    type Node = NodeId;
    type Weight = W;
    type Neighbours = OutgoingIter<'a, W>;

    fn weight(&self, from: &NodeId, to: &NodeId) -> Option<W> {
        match self.map.get(from) {
            Some(node) => self.nodes[*node].outgoing.get(to).map(|w| w.clone()),
            None => None,
        }
    }

    fn neighbours(&self, node: &NodeId) -> OutgoingIter<'a, W> {
        fn f(k: &usize) -> usize { *k }
        self.nodes[self.map[*node]].outgoing.keys().map(f)
    }
}

