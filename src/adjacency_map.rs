use Graph;

use num::{self, Zero};

use std::ops::Index;
use std::collections::HashMap;
use std::collections::hash_map::Keys;

pub type NodeId = usize;
type ListId = usize;
type Outgoing<'a, W> = Keys<'a, usize, W>;

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
    
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: W) {
        self.nodes[self.map[from]].outgoing.insert(to, weight);
    }
}

impl<V, W> Index<NodeId> for AdjacencyMap<V, W> {
    type Output = AdjacencyMapNode<V, W>;
    
    fn index(&self, index: &NodeId) -> &AdjacencyMapNode<V, W> {
        &self.nodes[self.map[*index]]
    }
}

impl<'a, N, W> Graph for &'a AdjacencyMap<N, W> where W: Zero + Clone {
    type Node = NodeId;
    type Weight = W;
    type Neighbours = Outgoing<'a, W>;
    
    fn weight(&self, from: &NodeId, to: &NodeId) -> Option<W> {
        match self.map.get(from) {
            Some(node) => self.nodes[*node].outgoing.get(to).map(|w| w.clone()),
            None => None,
        }
    }
    
    fn neighbours(&self, node: &NodeId) -> Outgoing<'a, W> {
        self.nodes[*node].outgoing.keys()
    }
    
    fn heuristic(&self, _: &NodeId, _: &NodeId) -> W {
        num::zero()
    }
}
