use crate::shared::graph::Graph;
use crate::shared::structures::Day;
use std::collections::HashMap;

pub fn day_09() -> Day {
    Day::new(
        9,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    parse_input(input).path_min().0.to_string()
}

fn part2(input: &str) -> String {
    parse_input(input).path_max().0.to_string()
}

type Edge<'a> = (&'a str, &'a str, usize);

fn parse_input(input: &str) -> Graph {
    let mut cities: HashMap<&str, usize> = HashMap::new();
    let mut city_index: usize = 0;
    let mut edges: Vec<Edge> = Vec::new();

    for line in input.trim().lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let city1 = line[0];
        let city2 = line[2];
        let distance = line[4].parse::<usize>().unwrap();
        edges.push((city1, city2, distance));
        if !cities.contains_key(city1) {
            cities.insert(city1, city_index);
            city_index += 1;
        }
        if !cities.contains_key(city2) {
            cities.insert(city2, city_index);
            city_index += 1;
        }
    }

    let mut graph = Graph::new(city_index);

    for edge in edges {
        graph.set_edge(
            cities[edge.0],
            cities[edge.1],
            isize::try_from(edge.2).unwrap(),
        );
        graph.set_edge(
            cities[edge.1],
            cities[edge.0],
            isize::try_from(edge.2).unwrap(),
        );
    }

    graph
}
