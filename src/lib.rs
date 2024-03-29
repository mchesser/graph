//! # Graph
//!
//! A WIP library for graph representations and algorithms

pub mod adjacency_map;
pub mod shortest_path;
pub mod minimum_spanning_tree;
pub mod traversal;

pub use adjacency_map::AdjacencyMap;

pub trait Graph {
    type NodeId;
    type Edge;
    type Weight;
    type OutgoingEdgesIter: Iterator<Item=Self::Edge>;

    fn target(&self, edge: &Self::Edge) -> Option<Self::NodeId>;
    fn weight(&self, edge: &Self::Edge) -> Self::Weight;
    fn outgoing_edges(&self, node: &Self::NodeId) -> Self::OutgoingEdgesIter;
}
