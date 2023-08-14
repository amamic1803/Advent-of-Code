#![allow(dead_code)]

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;


pub(crate) struct Graph {
    vertices: usize,
    edges: Vec<Vec<Option<isize>>>,
}

impl Graph {
    pub(crate) fn new(vertices: usize) -> Self {
        let mut edges = vec![vec![Some(0); vertices]; vertices];
        #[allow(clippy::needless_range_loop)]
        for i in 0..vertices {
            edges[i][i] = None;
        }

        Self {
            vertices,
            edges,
        }
    }

    pub(crate) fn set_edge(&mut self, from: usize, to: usize, weight: isize) {
        assert_ne!(from, to);
        self.edges[from][to] = Some(weight);
    }

    pub(crate) fn tsp(&mut self, starting_point: usize) -> (isize, Vec<usize>) {
        assert!(starting_point < self.vertices);

        let mut min_cost = isize::MAX;
        let mut min_path = Vec::with_capacity(self.vertices + 1);
        let mut queue: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        let rows_minis: Vec<isize> = self.edges.iter().map(|row| row.iter().flatten().copied().min().unwrap()).collect();
        let limit: isize = rows_minis.iter().sum();

        queue.push(Reverse(Node::new(limit, {
            let mut vec = Vec::with_capacity(self.vertices + 1);
            vec.push(starting_point);
            vec
        })));

        while !queue.is_empty() && queue.peek().unwrap().0.limit < min_cost {
            let mut current_node = queue.pop().unwrap().0;
            if current_node.path.len() == self.vertices {
                current_node.limit -= rows_minis[current_node.path[current_node.path.len() - 1]];
                current_node.limit += self.edges[current_node.path[current_node.path.len() - 1]][starting_point].unwrap();
                current_node.path.push(starting_point);

                if current_node.limit < min_cost {
                    min_cost = current_node.limit;
                    min_path = current_node.path.clone();
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

    pub(crate) fn tsp_max(&mut self, starting_point: usize) -> (isize, Vec<usize>) {
        assert!(starting_point < self.vertices);

        let mut max_cost = isize::MIN;
        let mut max_path = Vec::with_capacity(self.vertices + 1);
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let rows_maxes: Vec<isize> = self.edges.iter().map(|row| row.iter().flatten().copied().max().unwrap()).collect();
        let limit: isize = rows_maxes.iter().sum();

        queue.push(Node::new(limit, {
            let mut vec = Vec::with_capacity(self.vertices + 1);
            vec.push(starting_point);
            vec
        }));

        while !queue.is_empty() && queue.peek().unwrap().limit > max_cost {
            let mut current_node = queue.pop().unwrap();
            if current_node.path.len() == self.vertices {
                current_node.limit -= rows_maxes[current_node.path[current_node.path.len() - 1]];
                current_node.limit += self.edges[current_node.path[current_node.path.len() - 1]][starting_point].unwrap();
                current_node.path.push(starting_point);

                if current_node.limit > max_cost {
                    max_cost = current_node.limit;
                    max_path = current_node.path.clone();
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

    pub(crate) fn backtracking(&self, starting_point: usize) -> (isize, Vec<usize>) {
        assert!(starting_point < self.vertices);
        let mut min_cost = isize::MAX;
        let mut min_path = Vec::with_capacity(self.vertices + 1);
        let mut current_path = vec![starting_point];
        let current_cost = 0;
        self.backtracking_helper(&mut current_path, current_cost, &mut min_path, &mut min_cost);
        (min_cost, min_path)
    }

    fn backtracking_helper(&self, current_path: &mut Vec<usize>, mut current_cost: isize, min_path: &mut Vec<usize>, min_cost: &mut isize) {
        if current_path.len() == self.vertices {
            current_cost += self.edges[current_path[current_path.len() - 1]][current_path[0]].unwrap();
            current_path.push(current_path[0]);
            if current_cost < *min_cost {
                *min_cost = current_cost;
                *min_path = current_path.clone();
            }
            current_path.pop();
        } else {
            for i in 0..self.vertices {
                if !current_path.contains(&i) {
                    current_path.push(i);
                    self.backtracking_helper(current_path, current_cost + self.edges[current_path[current_path.len() - 2]][i].unwrap(), min_path, min_cost);
                    current_path.pop();
                }
            }
        }
    }

    pub(crate) fn backtracking_max(&self, starting_point: usize) -> (isize, Vec<usize>) {
        assert!(starting_point < self.vertices);
        let mut max_cost = isize::MIN;
        let mut max_path = Vec::with_capacity(self.vertices + 1);
        let mut current_path = vec![starting_point];
        let current_cost = 0;
        self.backtracking_max_helper(&mut current_path, current_cost, &mut max_path, &mut max_cost);
        (max_cost, max_path)
    }

    fn backtracking_max_helper(&self, current_path: &mut Vec<usize>, mut current_cost: isize, max_path: &mut Vec<usize>, max_cost: &mut isize) {
        if current_path.len() == self.vertices {
            current_cost += self.edges[current_path[current_path.len() - 1]][current_path[0]].unwrap();
            current_path.push(current_path[0]);
            if current_cost > *max_cost {
                *max_cost = current_cost;
                *max_path = current_path.clone();
            }
            current_path.pop();
        } else {
            for i in 0..self.vertices {
                if !current_path.contains(&i) {
                    current_path.push(i);
                    self.backtracking_max_helper(current_path, current_cost + self.edges[current_path[current_path.len() - 2]][i].unwrap(), max_path, max_cost);
                    current_path.pop();
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    limit: isize,
    path: Vec<usize>,
}

impl Node {
    fn new(limit: isize, path: Vec<usize>) -> Self {
        Self {
            limit,
            path,
        }
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