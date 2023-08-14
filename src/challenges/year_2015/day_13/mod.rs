use crate::challenges::Day;
use crate::shared::tsp::Graph;
use std::collections::HashMap;

pub(crate) fn day_13() -> Day {
    Day::new (
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    println!("{}", parse_input(input).tsp_max(0).0);
}

fn part2(input: &str) {
    println!("{}", input);
}


fn parse_input(input: &str) -> Graph {
    let mut names: Vec<&str> = Vec::new();
    let mut weights: HashMap<(&str, &str), isize> = HashMap::new();
    let mut weights_vec = Vec::new();
    for line in input.trim().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let name1 = words[0];
        let name2 = words[10].trim_end_matches('.');
        let value = words[3].parse::<isize>().unwrap() * if words[2] == "gain" { 1 } else { -1 };

        if !names.contains(&name1) {
            names.push(name1);
        }
        if !names.contains(&name2) {
            names.push(name2);
        }

        weights_vec.push((name1, name2, value));
    }

    for (name1, name2, value) in weights_vec {
        if weights.contains_key(&(name1, name2)) {
            *weights.get_mut(&(name1, name2)).unwrap() += value;
        } else if weights.contains_key(&(name2, name1)) {
            *weights.get_mut(&(name2, name1)).unwrap() += value;
        } else {
            weights.insert((name1, name2), value);
        }
    }

    let mut graph = Graph::new(names.len());
    for (key, val) in weights {
        let index1 = names.iter().position(|&x| x == key.0).unwrap();
        let index2 = names.iter().position(|&x| x == key.1).unwrap();
        graph.set_edge(index1, index2, val);
        graph.set_edge(index2, index1, val);
    }

    graph
}
