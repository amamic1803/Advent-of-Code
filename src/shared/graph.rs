//! A module for solving Graph problems.
//!
//! *List of problems:*
//! - Shortest Hamiltonian circle (TSP)
//! - Longest Hamiltonian circle
//! - Shortest Hamiltonian path
//! - Longest Hamiltonian path

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Eq, PartialEq)]
pub struct Graph {
    adj_list: HashMap<Vertex, Vec<(Vertex, isize)>>,
}
impl Graph {
    pub fn new() -> Self {
        Self { adj_list: HashMap::new() }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self { adj_list: HashMap::with_capacity(capacity) }
    }

    pub fn set_edge(&mut self, vertex1: Vertex, vertex2: Vertex, value: isize) {
        if !self.adj_list.contains_key(&vertex2) {
            panic!("vertex2 not present in the graph.");
        }

        match self.adj_list.get_mut(&vertex1) {
            Some(edges) => match edges.iter().position(|(id, _)| id == &vertex2) {
                Some(pos) => edges[pos].1 = value,
                None => edges.push((vertex2, value)),
            },
            None => panic!("vertex1 not present in the graph."),
        }
    }

    pub fn set_edge_undirected(&mut self, vertex1: Vertex, vertex2: Vertex, value: isize) {
        self.set_edge(vertex1, vertex2, value);
        self.set_edge(vertex2, vertex1, value);
    }

    pub fn get_edge(&self, vertex1: Vertex, vertex2: Vertex) -> isize {
        if !self.adj_list.contains_key(&vertex2) {
            panic!("vertex2 not present in the graph.");
        }

        match self.adj_list.get(&vertex1) {
            Some(edges) => match edges.iter().position(|(id, _)| id == &vertex2) {
                Some(pos) => edges[pos].1,
                None => panic!("Edge not present in the graph."),
            },
            None => panic!("vertex1 not present in the graph."),
        }
    }

    pub fn new_vertex(&mut self) -> Vertex {
        // find empty id
        let mut i = 0;
        while self.adj_list.contains_key(&Vertex { id: i }) {
            i += 1;
        }

        // create and add vertex
        let vertex = Vertex::new(i);
        self.add_vertex(vertex);

        // return vertex
        vertex
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        if self.adj_list.contains_key(&vertex) {
            panic!("Vertex is already present in the graph.");
        } else {
            self.adj_list.insert(vertex, Vec::new());
        }
    }
    
    pub fn remove_vertex(&mut self, vertex: Vertex) -> bool {
        match self.adj_list.remove(&vertex) {
            Some(_) => {
                for edges in self.adj_list.values_mut() {
                    edges.retain(|(id, _)| id != &vertex);
                }
                true
            }
            None => false,
        }
    }
    
    pub fn vertices<'a>(&'a self) -> impl Iterator<Item=Vertex> + 'a {
        self.adj_list.keys().copied()
    }

    pub fn hamiltonian_cycle_min(&self) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // define node structure used in the algorithm
        #[derive(Clone, Eq, PartialEq)]
        struct Node {
            min_cost: isize,    // minimum cost for whole cycle following this node
            path: Vec<Vertex>,  // path from starting node to this one
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.min_cost.cmp(&other.min_cost).reverse())  // reverse comparison because BinaryHeap is max, but we need minimum min_path first
            }
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        // initialize cycle and minimum cost
        let mut min_cycle = Vec::new();
        let mut min_cost = isize::MAX;

        // find minimum edge weight from every vertex
        let min_edges =
            self.adj_list
                .iter()
                .map(|(key, value)|
                    (*key, value.iter().map(|edge| edge.1).min().expect("Invalid graph! (vertex with no edges)"))
                )
                .collect::<HashMap<_, _>>();

        // priority queue
        // nodes with smaller min_cost are popped first
        let mut queue = BinaryHeap::new();

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as starting point
        // because the cycle can be rotated to start from any vertex
        let start_node = Node {
            min_cost: min_edges.values().sum(),
            path: vec![*self.adj_list.keys().next().unwrap()],
        };

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until all are processed
        // or the min_cost for popped node is greater than absolute min_cost
        // (all other nodes also have bigger min_cost since this is priority queue)
        while let Some(mut node) = queue.pop() {
            if node.min_cost > min_cost {
                break;
            }

            if node.path.len() == self.adj_list.len() {
                // if node contains path with the number of vertices equal to total number of vertices,
                // process the final edge (last vertex -> first vertex), and update min_cost and min_cycle if necessary

                let first_vertex = node.path.first().unwrap();
                let last_vertex = node.path.last().unwrap();
                node.min_cost -= min_edges[last_vertex];
                node.min_cost += self.adj_list[last_vertex]
                    .iter()
                    .find(|(id, _)| id == first_vertex)
                    .unwrap()
                    .1;

                if node.min_cost < min_cost {
                    min_cost = node.min_cost;
                    min_cycle = node.path;
                }
            } else {
                // if node contains path with fewer vertices than total,
                // consider all possible moves to next vertex along edge
                // (if that vertex isn't already visited, in the nodes path)
                // for each possible move, clone node, update min_cost and path, add to queue
                let last_vertex = node.path.last().unwrap();
                for (other, weight) in &self.adj_list[last_vertex] {
                    if !node.path.contains(other) {
                        let mut new_node = node.clone();
                        new_node.min_cost -= min_edges[last_vertex];
                        new_node.min_cost += weight;
                        if new_node.min_cost < min_cost {
                            new_node.path.push(*other);
                            queue.push(new_node)
                        }
                    }
                }
            }
        }

        // if min_cost is still at its initial value, no cycle was found
        // else return the minimum cost and the cycle
        if min_cost == isize::MAX {
            panic!("No cycle found!");
        } else {
            (min_cost, min_cycle)
        }
    }

    pub fn hamiltonian_cycle_max(&self) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // define node structure used in the algorithm
        #[derive(Clone, Eq, PartialEq)]
        struct Node {
            max_cost: isize,    // maximum cost for whole cycle following this node
            path: Vec<Vertex>,  // path from starting node to this one
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.max_cost.cmp(&other.max_cost).reverse())
            }
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        // initialize cycle and maximum cost
        let mut max_cycle = Vec::new();
        let mut max_cost = isize::MIN;

        // find maximum edge weight from every vertex
        let max_edges =
            self.adj_list
                .iter()
                .map(|(key, value)|
                (*key, value.iter().map(|edge| edge.1).max().expect("Invalid graph! (vertex with no edges)"))
                )
                .collect::<HashMap<_, _>>();

        // priority queue
        // nodes with bigger max_cost are popped first
        let mut queue = BinaryHeap::new();

        // take random vertex as starting point
        // it doesn't matter which vertex is chosen as starting point
        // because the cycle can be rotated to start from any vertex
        let start_node = Node {
            max_cost: max_edges.values().sum(),
            path: vec![*self.adj_list.keys().next().unwrap()],
        };

        // add starting node to the queue
        queue.push(start_node);

        // process nodes until all are processed
        // or the max_cost for popped node is less than absolute max_cost
        // (all other nodes also have smaller max_cost since this is priority queue)
        while let Some(mut node) = queue.pop() {
            if node.max_cost < max_cost {
                break;
            }

            if node.path.len() == self.adj_list.len() {
                // if node contains path with the number of vertices equal to total number of vertices,
                // process the final edge (last vertex -> first vertex), and update max_cost and max_cycle if necessary

                let first_vertex = node.path.first().unwrap();
                let last_vertex = node.path.last().unwrap();
                node.max_cost -= max_edges[last_vertex];
                node.max_cost += self.adj_list[last_vertex]
                    .iter()
                    .find(|(id, _)| id == first_vertex)
                    .unwrap()
                    .1;

                if node.max_cost > max_cost {
                    max_cost = node.max_cost;
                    max_cycle = node.path;
                }
            } else {
                // if node contains path with fewer vertices than total,
                // consider all possible moves to next vertex along edge
                // (if that vertex isn't already visited, in the nodes path)
                // for each possible move, clone node, update max_cost and path, add to queue
                let last_vertex = node.path.last().unwrap();
                for (other, weight) in &self.adj_list[last_vertex] {
                    if !node.path.contains(other) {
                        let mut new_node = node.clone();
                        new_node.max_cost -= max_edges[last_vertex];
                        new_node.max_cost += weight;
                        if new_node.max_cost > max_cost {
                            new_node.path.push(*other);
                            queue.push(new_node)
                        }
                    }
                }
            }
        }

        // if max_cost is still at its initial value, no cycle was found
        // else return the maximum cost and the cycle
        if max_cost == isize::MIN {
            panic!("No cycle found!");
        } else {
            (max_cost, max_cycle)
        }
    }

    pub fn hamiltonian_path_min(&mut self) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // existing vertices
        let vertices = self.vertices().collect::<Vec<_>>();

        // add new vertex
        let added_vertex = self.new_vertex();

        // set all edges to/from the added vertex to 0
        for vertex in &vertices {
            self.adj_list.get_mut(vertex).unwrap().push((added_vertex, 0));
        }
        self.adj_list.get_mut(&added_vertex).unwrap().extend(vertices.iter().map(|vertex| (*vertex, 0)));

        // find minimum hamiltonian cycle
        let (min_cost, mut min_path) = self.hamiltonian_cycle_min();

        // since added_vertex edges are 0, min_cost is correct
        // min_path is actually min_cycle that needs to be transformed into min_path
        // added_vertex must be removed from it, and adjacent vertices must be first and last vertices in the path

        // find current position of the added_vertex
        let added_vertex_pos = min_path.iter().position(|vertex| vertex == &added_vertex).unwrap();

        // rotate min_path so that the added_vertex is at index 0
        min_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        min_path.remove(0);

        // return min_cost and min_path
        (min_cost, min_path)
    }

    pub fn hamiltonian_path_max(&mut self) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }

        // existing vertices
        let vertices = self.vertices().collect::<Vec<_>>();

        // add new vertex
        let added_vertex = self.new_vertex();

        // set all edges to/from the added vertex to 0
        for vertex in &vertices {
            self.adj_list.get_mut(vertex).unwrap().push((added_vertex, 0));
        }
        self.adj_list.get_mut(&added_vertex).unwrap().extend(vertices.iter().map(|vertex| (*vertex, 0)));

        // find maximum hamiltonian cycle
        let (max_cost, mut max_path) = self.hamiltonian_cycle_max();

        // since added_vertex edges are 0, max_cost is correct
        // max_path is actually max_cycle that needs to be transformed into max_path
        // added_vertex must be removed from it, and adjacent vertices must be first and last vertices in the path

        // find current position of the added_vertex
        let added_vertex_pos = max_path.iter().position(|vertex| vertex == &added_vertex).unwrap();

        // rotate max_path so that the added_vertex is at index 0
        max_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        max_path.remove(0);

        // return min_cost and min_path
        (max_cost, max_path)
    }

    pub fn hamiltonian_path_fixed_ends_min(&mut self, end1: Vertex, end2: Vertex) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }
        if !self.adj_list.contains_key(&end1) {
            panic!("end1 not present in the graph.");
        }
        if !self.adj_list.contains_key(&end2) {
            panic!("end2 not present in the graph.");
        }

        // add new vertex
        let added_vertex = self.new_vertex();

        // set between start and added_vertex, added_vertex and end, to 0
        self.adj_list.get_mut(&added_vertex).unwrap().push((end1, 0));
        self.adj_list.get_mut(&end1).unwrap().push((added_vertex, 0));
        self.adj_list.get_mut(&added_vertex).unwrap().push((end2, 0));
        self.adj_list.get_mut(&end2).unwrap().push((added_vertex, 0));

        // find minimum hamiltonian cycle
        let (min_cost, mut min_path) = self.hamiltonian_cycle_min();

        // since added_vertex edges are 0, min_cost is correct
        // min_path is actually min_cycle that needs to be transformed into min_path
        // added_vertex must be removed from it, and adjacent vertices
        // (guaranteed to be end1, end2) must be first and last vertices in the path

        // find current position of the added_vertex
        let added_vertex_pos = min_path.iter().position(|vertex| vertex == &added_vertex).unwrap();

        // rotate min_path so that the added_vertex is at index 0
        min_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        min_path.remove(0);

        // return min_cost and min_path
        (min_cost, min_path)
    }

    pub fn hamiltonian_path_fixed_ends_max(&mut self, end1: Vertex, end2: Vertex) -> (isize, Vec<Vertex>) {
        if self.adj_list.len() < 2 {
            panic!("The graph must contain at least 2 vertices.");
        }
        if !self.adj_list.contains_key(&end1) {
            panic!("end1 not present in the graph.");
        }
        if !self.adj_list.contains_key(&end2) {
            panic!("end2 not present in the graph.");
        }

        // add new vertex
        let added_vertex = self.new_vertex();

        // set between start and added_vertex, added_vertex and end, to 0
        self.adj_list.get_mut(&added_vertex).unwrap().push((end1, 0));
        self.adj_list.get_mut(&end1).unwrap().push((added_vertex, 0));
        self.adj_list.get_mut(&added_vertex).unwrap().push((end2, 0));
        self.adj_list.get_mut(&end2).unwrap().push((added_vertex, 0));

        // find maximum hamiltonian cycle
        let (max_cost, mut max_path) = self.hamiltonian_cycle_max();

        // since added_vertex edges are 0, max_cost is correct
        // max_path is actually max_cycle that needs to be transformed into max_path
        // added_vertex must be removed from it, and adjacent vertices
        // (guaranteed to be end1, end2) must be first and last vertices in the path

        // find current position of the added_vertex
        let added_vertex_pos = max_path.iter().position(|vertex| vertex == &added_vertex).unwrap();

        // rotate max_path so that the added_vertex is at index 0
        max_path.rotate_left(added_vertex_pos);

        // remove added_vertex
        max_path.remove(0);

        // return max_cost and max_path
        (max_cost, max_path)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Vertex {
    id: usize,
}
impl Vertex {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
    pub fn id(&self) -> usize {
        self.id
    }
}
