//! # Graph
//!
//! A WIP library for graph representations and algorithms
#![feature(core)]

extern crate num;

pub mod shortest_path;

pub trait Graph {
    type Node;
    type Weight;
    type Neighbours;

    fn weight(&self, from: &Self::Node, to: &Self::Node) -> Option<Self::Weight>;
    fn neighbours(&self, node: &Self::Node) -> Self::Neighbours;
    fn heuristic(&self, from: &Self::Node, to: &Self::Node) -> Self::Weight;
}
