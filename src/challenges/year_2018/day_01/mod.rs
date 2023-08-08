use crate::challenges::Day;
use std::collections::HashSet;

pub(crate) fn day_01() -> Day {
    Day::new(
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    let input = parse_input(input);

    println!("{:?}", input.iter().sum::<isize>());
}

fn part2(input: &str) {
    let input = parse_input(input);
    let mut i: usize = 0;
    let mut encountered_frequencies: HashSet<isize> = HashSet::new();
    let mut frequency: isize = 0;

    loop {
        frequency += input[i];
        if encountered_frequencies.insert(frequency) {
            i = (i + 1) % input.len();
        } else {
            println!("{:?}", frequency);
            break;
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