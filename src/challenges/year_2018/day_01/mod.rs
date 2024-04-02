use crate::shared::structures::Day;
use std::collections::HashSet;

pub fn day_01() -> Day {
    Day::new(
        1,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let input = parse_input(input);

    input.iter().sum::<isize>().to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let mut i: usize = 0;
    let mut encountered_frequencies: HashSet<isize> = HashSet::new();
    let mut frequency: isize = 0;

    loop {
        frequency += input[i];
        if encountered_frequencies.insert(frequency) {
            i = (i + 1) % input.len();
        } else {
            return frequency.to_string();
        }
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}
