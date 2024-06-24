use crate::shared::structures::Day;

pub fn day_22() -> Day {
    Day::new(22, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    Grid::new(input).viable_pairs().to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

struct Grid {
    nodes: Vec<Vec<Node>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut nodes = Vec::new();
        for line in input.lines().skip(2) {
            let node = Node::new(line);
            if node.y >= nodes.len() {
                nodes.push(Vec::new());
            }
            nodes[node.y].push(node);
        }

        Self { nodes }
    }

    fn viable_pairs(&self) -> usize {
        let mut pairs = 0;
        for node in self.nodes.iter().flat_map(|row| row.iter()) {
            for node2 in self.nodes.iter().flat_map(|row| row.iter()) {
                if (node.x != node2.x || node.y != node2.y) && node.used != 0 && node.used <= node2.avail {
                    pairs += 1;
                }
            }
        }
        pairs
    }
}

struct Node {
    x: usize,
    y: usize,
    size: u16,
    used: u16,
    avail: u16,
    use_percent: u16,
}
impl Node {
    fn new(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let path = parts.next().unwrap();
        let mut path_parts = path.split('-');
        let _ = path_parts.next().unwrap();
        let x = path_parts.next().unwrap().trim_start_matches('x').parse().unwrap();
        let y = path_parts.next().unwrap().trim_start_matches('y').parse().unwrap();

        let size = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
        let used = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
        let avail = parts.next().unwrap().trim_end_matches('T').parse().unwrap();
        let use_percent = parts.next().unwrap().trim_end_matches('%').parse().unwrap();

        Self {
            x,
            y,
            size,
            used,
            avail,
            use_percent,
        }
    }
}
