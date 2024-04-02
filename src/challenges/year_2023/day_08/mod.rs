use crate::shared::structures::Day;

pub fn day_08() -> Day {
    Day::new(
        8,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

use crate::shared::math::lcm_multiple;

fn part1(input: &str) -> String {
    let tree = Tree::new(input);
    tree.simulate_1().to_string()
}

fn part2(input: &str) -> String {
    let tree = Tree::new(input);
    tree.simulate_2().to_string()
}

struct Node<'a> {
    id: &'a str,
    left: usize,
    right: usize,
}
impl<'a> Node<'a> {
    fn new(id: &'a str, left: usize, right: usize) -> Self {
        Self { id, left, right }
    }
}
struct Tree<'a> {
    nodes: Vec<Node<'a>>,
    root: usize,
    instructions: Vec<bool>, // false = left, true = right
}
impl<'a> Tree<'a> {
    fn new(input: &'a str) -> Self {
        let mut input_lines = input.trim().lines();
        let instructions = input_lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => false,
                'R' => true,
                _ => panic!("Invalid input"),
            })
            .collect::<Vec<bool>>();

        input_lines.next(); // Skip empty line

        let mut nodes_input = Vec::new();
        for line in input_lines {
            let (node_id, left_right) = line.split_once(" = ").unwrap();
            let (left, right) = left_right
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(", ")
                .unwrap();
            nodes_input.push((node_id, left, right));
        }

        let mut nodes = Vec::new();
        for node_info in &nodes_input {
            nodes.push(Node::new(
                node_info.0,
                nodes_input
                    .iter()
                    .position(|(id, _, _)| id == &node_info.1)
                    .unwrap(),
                nodes_input
                    .iter()
                    .position(|(id, _, _)| id == &node_info.2)
                    .unwrap(),
            ));
        }

        let root = nodes_input
            .iter()
            .position(|(id, _, _)| id == &"AAA")
            .unwrap();

        Self {
            nodes,
            root,
            instructions,
        }
    }

    fn simulate_1(&self) -> u64 {
        let mut steps = 0;
        let mut current_node = self.root;
        while self.nodes[current_node].id != "ZZZ" {
            match self.instructions[steps as usize % self.instructions.len()] {
                false => current_node = self.nodes[current_node].left,
                true => current_node = self.nodes[current_node].right,
            }
            steps += 1;
        }
        steps
    }

    fn simulate_2(&self) -> u64 {
        // input is specifically designed for this method to work
        // (there is no general solution for this problem)
        // for each starting node that ends with 'A', there is a unique ending node that ends with 'Z' that forms a unique cycle
        // and each cycle starts immediately after starting node, therefore the length of the cycle is the length of the path to the ending node
        // obviously if we calculate the length of each cycle and find the least common multiple of all of them, we will get the answer

        let mut cycle_lens = Vec::new();

        for i in 0..self.nodes.len() {
            if self.nodes[i].id.ends_with('A') {
                let mut steps = 0;
                let mut current_node = i;

                while !self.nodes[current_node].id.ends_with('Z') {
                    match self.instructions[steps as usize % self.instructions.len()] {
                        false => current_node = self.nodes[current_node].left,
                        true => current_node = self.nodes[current_node].right,
                    }
                    steps += 1;
                }

                cycle_lens.push(steps);
            }
        }

        lcm_multiple(&cycle_lens)
    }
}
