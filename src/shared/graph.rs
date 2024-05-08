//! A module for solving Graph problems.
//!
//! *List of problems:*
//! - Shortest Hamiltonian circle (TSP)
//! - Longest Hamiltonian circle
//! - Shortest Hamiltonian path
//! - Longest Hamiltonian path

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Graph {
    vertices: usize,
    edges: Vec<Vec<Option<isize>>>,
}

impl Graph {
    /// Instantiate new graph with given number of vertices.
    /// Vertices are indexed from 0 to `vertices - 1`.
    /// All edges are initialized with weight 0.
    pub fn new(vertices: usize) -> Self {
        assert!(vertices > 0);
        let mut edges = vec![vec![Some(0); vertices]; vertices];
        #[allow(clippy::needless_range_loop)]
        for i in 0..vertices {
            edges[i][i] = None;
        }

        Self { vertices, edges }
    }

    /// Set edge weight.
    pub fn set_edge(&mut self, from: usize, to: usize, weight: isize) {
        assert_ne!(from, to);
        self.edges[from][to] = Some(weight);
    }

    /// Get edge weight.
    pub fn get_edge(&self, from: usize, to: usize) -> isize {
        assert_ne!(from, to);
        self.edges[from][to].unwrap()
    }

    /// Find shortest Hamiltonian circle.
    pub fn circle_min(&self, start_point: usize) -> (isize, Vec<usize>) {
        assert!(start_point < self.vertices);

        let mut min_cost = isize::MAX;
        let mut min_path = Vec::with_capacity(self.vertices + 1);
        let mut queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        let rows_minis: Vec<isize> = self.edges.iter().map(|row| row.iter().flatten().copied().min().unwrap()).collect();
        let limit: isize = rows_minis.iter().sum();

        queue.push(Reverse(Node::new(limit, {
            let mut vec = Vec::with_capacity(self.vertices + 1);
            vec.push(start_point);
            vec
        })));

        while !queue.is_empty() && queue.peek().unwrap().0.limit < min_cost {
            let mut current_node = queue.pop().unwrap().0;
            if current_node.path.len() == self.vertices {
                current_node.limit -= rows_minis[current_node.path[current_node.path.len() - 1]];
                current_node.limit += self.edges[current_node.path[current_node.path.len() - 1]][start_point].unwrap();
                current_node.path.push(start_point);

                if current_node.limit < min_cost {
                    min_cost = current_node.limit;
                    min_path.clone_from(&current_node.path);
                }
            } else {
                for i in 0..self.vertices {
                    if !current_node.path.contains(&i) {
                        let mut new_node = current_node.clone();
                        new_node.limit -= rows_minis[new_node.path[new_node.path.len() - 1]];
                        new_node.limit += self.edges[new_node.path[new_node.path.len() - 1]][i].unwrap();
                        if new_node.limit < min_cost {
                            new_node.path.push(i);
                            queue.push(Reverse(new_node));
                        }
                    }
                }
            }
        }

        (min_cost, min_path)
    }

    /// Find longest Hamiltonian circle.
    pub fn circle_max(&self, start_point: usize) -> (isize, Vec<usize>) {
        assert!(start_point < self.vertices);

        let mut max_cost = isize::MIN;
        let mut max_path = Vec::with_capacity(self.vertices + 1);
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let rows_maxes: Vec<isize> = self.edges.iter().map(|row| row.iter().flatten().copied().max().unwrap()).collect();
        let limit: isize = rows_maxes.iter().sum();

        queue.push(Node::new(limit, {
            let mut vec = Vec::with_capacity(self.vertices + 1);
            vec.push(start_point);
            vec
        }));

        while !queue.is_empty() && queue.peek().unwrap().limit > max_cost {
            let mut current_node = queue.pop().unwrap();
            if current_node.path.len() == self.vertices {
                current_node.limit -= rows_maxes[current_node.path[current_node.path.len() - 1]];
                current_node.limit += self.edges[current_node.path[current_node.path.len() - 1]][start_point].unwrap();
                current_node.path.push(start_point);

                if current_node.limit > max_cost {
                    max_cost = current_node.limit;
                    max_path.clone_from(&current_node.path);
                }
            } else {
                for i in 0..self.vertices {
                    if !current_node.path.contains(&i) {
                        let mut new_node = current_node.clone();
                        new_node.limit -= rows_maxes[new_node.path[new_node.path.len() - 1]];
                        new_node.limit += self.edges[new_node.path[new_node.path.len() - 1]][i].unwrap();
                        if new_node.limit > max_cost {
                            new_node.path.push(i);
                            queue.push(new_node);
                        }
                    }
                }
            }
        }

        (max_cost, max_path)
    }

    /// Find shortest Hamiltonian path.
    pub fn path_min(&self) -> (isize, Vec<usize>) {
        let mut new_graph = self.clone();
        new_graph.vertices += 1;
        for row in &mut new_graph.edges {
            row.push(Some(0));
        }
        new_graph.edges.push(vec![Some(0); new_graph.vertices + 1]);

        let (min_cost, mut min_path) = new_graph.circle_min(new_graph.vertices - 1);
        min_path.pop();
        min_path.remove(0);

        (min_cost, min_path)
    }

    /// Find longest Hamiltonian path.
    pub fn path_max(&self) -> (isize, Vec<usize>) {
        let mut new_graph = self.clone();
        new_graph.vertices += 1;
        for row in &mut new_graph.edges {
            row.push(Some(0));
        }
        new_graph.edges.push(vec![Some(0); new_graph.vertices + 1]);

        let (max_cost, mut max_path) = new_graph.circle_max(new_graph.vertices - 1);
        max_path.pop();
        max_path.remove(0);

        (max_cost, max_path)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    limit: isize,
    path: Vec<usize>,
}

impl Node {
    fn new(limit: isize, path: Vec<usize>) -> Self {
        Self { limit, path }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.limit.cmp(&other.limit))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.limit.cmp(&other.limit)
    }
}
