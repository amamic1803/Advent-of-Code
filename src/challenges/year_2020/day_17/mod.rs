use crate::shared::structures::Day;
use ndlife::Life;
use std::collections::HashSet;

pub fn day_17() -> Day {
    Day::new(17, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let birth_rules = [3].into_iter().collect();
    let survival_rules = [2, 3].into_iter().collect();
    let mut alive_cells = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                alive_cells.insert([x as i64, y as i64, 0]);
            }
        }
    }

    let mut life = Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
    for _ in 0..6 {
        life.next_generation();
    }

    life.alive_cells().len().to_string()
}

fn part2(input: &str) -> String {
    let birth_rules = [3].into_iter().collect();
    let survival_rules = [2, 3].into_iter().collect();
    let mut alive_cells = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                alive_cells.insert([x as i64, y as i64, 0, 0]);
            }
        }
    }

    let mut life = Life::new_with_alive_cells(birth_rules, survival_rules, alive_cells).unwrap();
    for _ in 0..6 {
        life.next_generation();
    }

    life.alive_cells().len().to_string()
}
