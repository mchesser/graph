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

pub fn a_star<G: Graph>(graph: &G, start: G::Node, end: G::Node,
    heuristic: Heuristic<G::Node, G::Weight>) -> Option<Vec<G::Node>>
    where G::Node: Hash<Hasher> + Eq + Clone,
          G::Neighbours: Iterator<Item=G::Node>,
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
struct PathNode<N, C> {
    node: N,
    parent: Option<N>,
    h_cost: C,
    g_cost: C,
}

impl<N, C: Zero + Add> PathNode<N, C> {
    pub fn new(node: N, parent: Option<N>) -> PathNode<N, C> {
        PathNode {
            node: node,
            h_cost: num::zero(),
            g_cost: num::zero(),
            parent: parent,
        }
    }
}

impl<N, C> PathNode<N, C> where C: Add<Output=C> + Clone {
    pub fn f_cost(&self) -> C {
        self.h_cost.clone() + self.g_cost.clone()
    }
}

impl<N, C> PartialEq for PathNode<N, C> where C: PartialEq + Add<Output=C> + Clone {
    fn eq(&self, other: &PathNode<N, C>) -> bool {
        self.f_cost() == other.f_cost()
    }
}
impl<N, C> Eq for PathNode<N, C> where C: PartialEq + Add<Output=C> + Clone {}

impl<N, C> PartialOrd for PathNode<N, C> where C: PartialOrd + Add<Output=C> + Clone {
    fn partial_cmp(&self, other: &PathNode<N, C>) -> Option<Ordering> {
        // Reverse the ordering so that it makes a min queue
        other.f_cost().partial_cmp(&self.f_cost())
    }
}
impl<N, C> Ord for PathNode<N, C> where C: Ord + Add<Output=C> + Clone {
    fn cmp(&self, other: &PathNode<N, C>) -> Ordering {
        // Reverse the ordering so that it makes a min queue
        other.f_cost().cmp(&self.f_cost())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;
    use std::num::SignedInt;
    use Graph;

    struct Grid {
        width: u32,
        height: u32,
        data: Vec<bool>,
    }

    impl Grid {
        fn new(width: u32, height: u32) -> Grid {
            Grid {
                width: width,
                height: height,
                data: iter::repeat(false).take((width as usize * height as usize)).collect(),
            }
        }

        fn get(&self, index: &(u32, u32)) -> bool {
            let &(x, y) = index;
            if x >= self.width || y >= self.height {
                true
            }
            else {
                self.data[y as usize * self.width as usize + x as usize]
            }
        }

        fn set(&mut self, index: &(u32, u32), value: bool) {
            let &(x, y) = index;
            if x >= self.width || y >= self.height {
                panic!("Index out of bounds");
            }

            let index = y as usize * self.width as usize + x as usize;
            self.data[index] = value;
        }

    }

    struct Adjacent {
        pos: (u32, u32),
        i: usize,
        width: u32,
        height: u32,
    }

    impl Iterator for Adjacent {
        type Item = (u32, u32);

        fn next(&mut self) -> Option<(u32, u32)> {
            static POSSIBLE_MOVES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
            let (x, y) = self.pos;
            loop {
                if self.i > 3 {
                    return None;
                }
                let (dx, dy) = POSSIBLE_MOVES[self.i];
                let (x, y) = (x as i32 + dx, y as i32 + dy);
                self.i += 1;
                if x >= 0 && y >= 0 && (x as u32) < self.width && (y as u32) < self.height {
                    return Some((x as u32, y as u32))
                }
            }
        }
    }

    impl Graph for Grid {
        type Node = (u32, u32);
        type Weight = usize;
        type Neighbours = Adjacent;

        fn weight(&self, _: &(u32, u32), to: &(u32, u32)) -> Option<usize> {
            if self.get(to) { None } else { Some(10) }
        }

        fn neighbours(&self, node: &(u32, u32)) -> Adjacent {
            Adjacent {
                pos: *node,
                i: 0,
                width: self.width,
                height: self.height
            }
        }
    }

    fn grid_heuristic(from: &(u32, u32), to: &(u32, u32)) -> usize {
        ((from.0 as i32 - to.0 as i32).abs() + (from.1 as i32 - to.0 as i32).abs()) as usize * 10
    }

    #[test]
    fn test_pathfinding_basic() {
        let grid = Grid::new(5, 5);
        let path = a_star(&grid, (0, 0), (4, 0), grid_heuristic);
        assert!(path.is_some());
        assert_eq!(path.unwrap(), vec![(4, 0), (3, 0), (2, 0), (1, 0), (0, 0)]);
    }

    #[test]
    fn test_pathfinding_off_screen() {
        let grid = Grid::new(5, 5);
        let path = a_star(&grid, (0, 0), (5, 0), grid_heuristic);
        assert!(path.is_none());
    }

    #[test]
    fn test_pathfinding_possible() {
        let mut grid = Grid::new(400, 200);
        for y in (0..100) {
            grid.set(&(7, y), true);
        }

        let path = a_star(&grid, (2, 2), (300, 50), grid_heuristic);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path[0], (300, 50));
        assert_eq!(path.last().unwrap(), &(2, 2));

        for (&(x1, y1), &(x2, y2)) in path.iter().zip(path.iter().skip(1)) {
            assert!(grid.get(&(x1, y1)) == false);
            assert!(((x2 as i32 - x1 as i32) + (y2 as i32 - y1 as i32)).abs() == 1);
        }
    }

    #[test]
    fn test_pathfinding_impossible() {
        let mut grid = Grid::new(5, 5);
        grid.set(&(3, 0), true);
        grid.set(&(4, 1), true);
        let path = a_star(&grid, (0, 0), (4, 0), grid_heuristic);
        assert!(path.is_none());
    }
}
