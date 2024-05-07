use crate::shared::chinese_remainder_theorem;
use crate::shared::structures::Day;
use std::cmp::Reverse;

pub fn day_15() -> Day {
    Day::new(15, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let discs = parse_input(input);
    solve(discs).to_string()
}

fn part2(input: &str) -> String {
    let discs = parse_input(input).chain([(11, 0)]);
    solve(discs).to_string()
}

fn solve(discs: impl Iterator<Item = (usize, usize)>) -> u64 {
    let mut congruences = Vec::new();
    for (i, disc) in discs.enumerate() {
        let rhs_value = (-(disc.1 as i64 + i as i64 + 1)).rem_euclid(disc.0 as i64);
        congruences.push((rhs_value as u64, disc.0 as u64));
    }

    congruences.sort_by_key(|&(_, modulus)| Reverse(modulus));
    chinese_remainder_theorem(&congruences)
}

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.lines().map(|line| {
        let positions = line.split_whitespace().nth(3).unwrap().parse().unwrap();
        let start = line.split_whitespace().nth(11).unwrap().trim_end_matches('.').parse().unwrap();
        (positions, start)
    })
}
