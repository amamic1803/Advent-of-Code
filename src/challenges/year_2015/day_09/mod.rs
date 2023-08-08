use crate::challenges::Day;
use std::collections::HashMap;

pub(crate) fn day_09() -> Day {
    Day::new (
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    let mut graph = Graph::new(input);
    println!("{}", graph.shortest_connecting_path());
}

fn part2(input: &str) {
    let mut graph = Graph::new(input);
    println!("{}", graph.longest_connecting_path());
}


struct Graph<'a> {
    nodes: Vec<Node<'a>>,
    edges: Edges<'a>,
    memoized_paths: HashMap<Vec<Node<'a>>, usize>,
    temp_vec: Vec<Node<'a>>,
}

impl<'a> Graph <'a> {
    fn new(input: &'a str) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Edges = Edges::new();

        for line in input.trim().lines() {
            let words: Vec<&str> = line.split_whitespace().collect();
            let from = Node::new(words[0]);
            let to = Node::new(words[2]);
            let distance = words[4].parse::<usize>().unwrap();

            if !nodes.contains(&from) {
                nodes.push(from);
            }
            if !nodes.contains(&to) {
                nodes.push(to);
            }
            edges.add_edge(from, to, distance);
        }

        Self {
            nodes,
            edges,
            memoized_paths: HashMap::new(),
            temp_vec: Vec::new(),
        }
    }

    fn shortest_connecting_path(&mut self) -> usize {
        let mut shortest_path: usize = usize::MAX;
        for i in 0..(self.nodes.len() - 1) {
            for j in (i + 1)..self.nodes.len() {
                self.memoized_paths.clear();
                let mut temp_path = vec![self.nodes[i]];
                let distance = self.shortest_connecting_path_recursive(&mut temp_path, self.nodes[j]);
                if distance < shortest_path {
                    shortest_path = distance;
                }
            }
        }
        shortest_path
    }

    fn shortest_connecting_path_recursive(&mut self, temp_path: &mut Vec<Node<'a>>, end: Node<'a>) -> usize {
        self.temp_vec = temp_path.clone();
        self.temp_vec.sort();
        if let Some(distance) = self.memoized_paths.get(&self.temp_vec) {
            return *distance;
        }

        if self.nodes.len() - temp_path.len() == 1 {
            let distance = self.edges.get_edge(temp_path[temp_path.len() - 1], end).unwrap();
            self.memoized_paths.insert(temp_path.clone(), distance);
            return distance;
        }

        let mut shortest_path: usize = usize::MAX;
        for i in 0..self.nodes.len() {
            if !temp_path.contains(&self.nodes[i]) && self.nodes[i] != end {
                temp_path.push(self.nodes[i]);
                let distance = self.shortest_connecting_path_recursive(temp_path, end) + self.edges.get_edge(temp_path[temp_path.len() - 2], temp_path[temp_path.len() - 1]).unwrap();
                temp_path.pop();
                if distance < shortest_path {
                    shortest_path = distance;
                }
            }
        }
        self.memoized_paths.insert(temp_path.clone(), shortest_path);

        shortest_path
    }

    fn longest_connecting_path(&mut self) -> usize {
        let mut longest_path: usize = 0;
        for i in 0..(self.nodes.len() - 1) {
            for j in (i + 1)..self.nodes.len() {
                self.memoized_paths.clear();
                let mut temp_path = vec![self.nodes[i]];
                let distance = self.longest_connecting_path_recursive(&mut temp_path, self.nodes[j]);
                if distance > longest_path {
                    longest_path = distance;
                }
            }
        }
        longest_path
    }

    fn longest_connecting_path_recursive(&mut self, temp_path: &mut Vec<Node<'a>>, end: Node<'a>) -> usize {
        self.temp_vec = temp_path.clone();
        self.temp_vec.sort();
        if let Some(distance) = self.memoized_paths.get(&self.temp_vec) {
            return *distance;
        }

        if self.nodes.len() - temp_path.len() == 1 {
            let distance = self.edges.get_edge(temp_path[temp_path.len() - 1], end).unwrap();
            self.memoized_paths.insert(temp_path.clone(), distance);
            return distance;
        }

        let mut longest_path: usize = 0;
        for i in 0..self.nodes.len() {
            if !temp_path.contains(&self.nodes[i]) && self.nodes[i] != end {
                temp_path.push(self.nodes[i]);
                let distance = self.longest_connecting_path_recursive(temp_path, end) + self.edges.get_edge(temp_path[temp_path.len() - 2], temp_path[temp_path.len() - 1]).unwrap();
                temp_path.pop();
                if distance > longest_path {
                    longest_path = distance;
                }
            }
        }
        self.memoized_paths.insert(temp_path.clone(), longest_path);

        longest_path
    }
}


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Node<'a> {
    name: &'a str,
}

impl<'a> Node<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
        }
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Edges<'a> {
    edges: HashMap<(Node<'a>, Node<'a>), usize>,
}

impl<'a> Edges<'a> {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: Node<'a>, to: Node<'a>, distance: usize) {
        if !(self.edges.contains_key(&(from, to)) || self.edges.contains_key(&(to, from))) {
            self.edges.insert((from, to), distance);
        }
    }

    fn get_edge(&self, from: Node<'a>, to: Node<'a>) -> Option<usize> {
        match self.edges.get(&(from, to)) {
            Some(distance) => Some(*distance),
            None => self.edges.get(&(to, from)).copied(),
        }
    }
}