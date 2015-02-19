//! # Graph
//!
//! A WIP library for graph representations and algorithms
#![feature(std_misc, hash)]

extern crate num;

pub mod adjacency_map;
pub mod shortest_path;

pub use adjacency_map::AdjacencyMap;

pub trait Graph {
    type NodeId;
    type Weight;
    type Neighbours;

    fn weight(&self, from: &Self::NodeId, to: &Self::NodeId) -> Option<Self::Weight>;
    fn neighbours(&self, node: &Self::NodeId) -> Self::Neighbours;
}
