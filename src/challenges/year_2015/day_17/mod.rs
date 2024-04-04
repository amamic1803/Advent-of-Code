use crate::shared::structures::Day;
use std::cmp::{min, Ordering, Reverse};
use std::collections::HashMap;

pub fn day_17() -> Day {
    Day::new(17, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut bottles = parse_input(input);
    bottles.sort_by_key(|value| Reverse(*value));
    let mut memoization = HashMap::new();

    count_combinations(0, 150, &bottles, &mut memoization).to_string()
}

fn part2(input: &str) -> String {
    let mut bottles = parse_input(input);
    bottles.sort_by_key(|value| Reverse(*value));
    let mut memoization = HashMap::new();

    count_least_combinations(0, 150, &bottles, &mut memoization).0.to_string()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.trim().lines().map(|num| num.parse::<usize>().unwrap()).collect()
}

fn count_combinations(i: usize, amount_left: usize, bottles: &[usize], memoization: &mut HashMap<(usize, usize), usize>) -> usize {
    match memoization.get(&(i, amount_left)) {
        Some(&value) => value,
        None => {
            let mut combinations: usize = 0;
            if i == bottles.len() - 1 {
                if amount_left == 0 || bottles[i] == amount_left {
                    combinations = 1;
                }
            } else {
                for curr_amount in 0..min((amount_left / bottles[i]) + 1, 2) {
                    combinations += count_combinations(i + 1, amount_left - curr_amount * bottles[i], bottles, memoization);
                }
            }
            memoization.insert((i, amount_left), combinations);
            combinations
        }
    }
}

fn count_least_combinations(i: usize, amount_left: usize, bottles: &[usize], memoization: &mut HashMap<(usize, usize), (usize, usize)>) -> (usize, usize) {
    match memoization.get(&(i, amount_left)) {
        Some(&value) => value,
        None => {
            let mut combinations: usize = 0;
            let mut least_bottles: usize = usize::MAX - 1;
            if i == bottles.len() - 1 {
                if amount_left == 0 {
                    combinations = 1;
                    least_bottles = 0;
                } else if amount_left == bottles[i] {
                    combinations = 1;
                    least_bottles = 1;
                }
            } else {
                for curr_amount in 0..min((amount_left / bottles[i]) + 1, 2) {
                    let result = count_least_combinations(i + 1, amount_left - curr_amount * bottles[i], bottles, memoization);
                    match (result.1 + curr_amount).cmp(&least_bottles) {
                        Ordering::Less => {
                            least_bottles = result.1 + curr_amount;
                            combinations = result.0;
                        }
                        Ordering::Equal => {
                            combinations += result.0;
                        }
                        Ordering::Greater => {}
                    }
                }
            }
            memoization.insert((i, amount_left), (combinations, least_bottles));
            (combinations, least_bottles)
        }
    }
}
