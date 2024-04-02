use crate::shared::structures::Day;

pub fn day_04() -> Day {
    Day::new(
        4,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    input
        .trim()
        .lines()
        .filter(|line| {
            let mut words = line.split_whitespace().collect::<Vec<_>>();
            let original_len = words.len();
            words.sort();
            words.dedup();
            original_len == words.len()
        })
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .trim()
        .lines()
        .filter(|line| {
            let mut words = line
                .split_whitespace()
                .map(|word| {
                    let mut chars = word.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars
                })
                .collect::<Vec<_>>();
            let original_len = words.len();
            words.sort();
            words.dedup();
            original_len == words.len()
        })
        .count()
        .to_string()
}
