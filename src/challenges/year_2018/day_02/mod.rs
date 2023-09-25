use crate::shared::structures::Day;
use std::collections::HashMap;

pub fn day_02() -> Day {
    Day::new(
        2,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let mut two_count = 0;
    let mut three_count = 0;

    for hash_map in parse_input_1(input) {
        let mut two_found = false;
        let mut three_found = false;
        for (_, count) in hash_map.iter() {
            match *count {
                2 => two_found = true,
                3 => three_found = true,
                _ => (),
            }
        }
        if two_found {
            two_count += 1;
        }
        if three_found {
            three_count += 1;
        }
    }

    (two_count * three_count).to_string()
}

fn part2(input: &str) -> String {
    let mut ids = parse_input_2(input);

    while ids.len() > 2 {
        let id = ids.pop().unwrap();
        for id2 in ids.iter() {
            if different_chars(id, id2) == 1 {
                return id
                    .chars()
                    .zip(id2.chars())
                    .filter(|(char1, char2)| char1 == char2)
                    .map(|(char1, _)| char1)
                    .collect();
            }
        }

    }

    if different_chars(ids[0], ids[1]) == 1 {
        return ids[0]
            .chars()
            .zip(ids[1].chars())
            .filter(|(char1, char2)| char1 == char2)
            .map(|(char1, _)| char1)
            .collect();
    } else {
        panic!("No solution found");
    }
}

fn parse_input_1(input: &str) -> Vec<HashMap<char, u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut hash_map = HashMap::new();
            for charachter in line.chars() {
                let count = hash_map.entry(charachter).or_insert(0);
                *count += 1;
            }
            hash_map
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn different_chars(input1: &str, input2: &str) -> u32 {
    let mut count = 0;
    for (char1, char2) in input1.chars().zip(input2.chars()) {
        if char1 != char2 {
            count += 1;
        }
    }
    count
}