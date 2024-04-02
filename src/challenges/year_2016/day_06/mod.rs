use crate::shared::structures::Day;
use std::collections::HashMap;

pub fn day_06() -> Day {
    Day::new(
        6,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    solve(input, false)
}

fn part2(input: &str) -> String {
    solve(input, true)
}

fn solve(input: &str, minimum: bool) -> String {
    let message_len = input.trim().lines().next().unwrap().trim().chars().count();
    let mut message: Vec<HashMap<char, usize>> = Vec::with_capacity(message_len);
    for _ in 0..message_len {
        message.push(HashMap::new());
    }

    for line in input.trim().lines() {
        for (i, c) in line.chars().enumerate() {
            let count = message[i].entry(c).or_insert(0);
            *count += 1;
        }
    }

    let mut result = String::new();

    for val in message {
        result.push(if minimum {
            *val.iter().min_by_key(|&(_, v)| v).unwrap().0
        } else {
            *val.iter().max_by_key(|&(_, v)| v).unwrap().0
        });
    }

    result
}
