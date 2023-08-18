use crate::shared::structures::Day;
use itertools::Itertools;

pub fn day_24() -> Day {
    Day::new (
        24,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let weights = parse_input(input);
    let total_weight: usize = weights.iter().sum();
    let group_weight = total_weight / 3;
    let mut min_qe = usize::MAX;

    for i in 0..(weights.len() / 3) {
        let mut found = false;
        for combination in weights.iter().combinations(i) {
            if combination.iter().copied().sum::<usize>() == group_weight {
                found = true;
                let qe = combination.into_iter().product::<usize>();
                if qe < min_qe {
                    min_qe = qe;
                }
            }
        }
        if found { break; }
    }

    min_qe.to_string()
}

fn part2(input: &str) -> String {
    let weights = parse_input(input);
    let total_weight: usize = weights.iter().sum();
    let group_weight = total_weight / 4;
    let mut min_qe = usize::MAX;

    for i in 0..(weights.len() / 4) {
        let mut found = false;
        for combination in weights.iter().combinations(i) {
            if combination.iter().copied().sum::<usize>() == group_weight {
                found = true;
                let qe = combination.into_iter().product::<usize>();
                if qe < min_qe {
                    min_qe = qe;
                }
            }
        }
        if found { break; }
    }

    min_qe.to_string()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}