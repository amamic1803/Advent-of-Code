use crate::{Day, Error};
use ndlife::Life;
use std::collections::HashSet;

pub struct Day17;
impl Day17 {
    pub fn new() -> Self {
        Self
    }
}
impl Day for Day17 {
    fn id(&self) -> usize {
        17
    }

    fn title(&self) -> &str {
        "Conway Cubes"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
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

        Ok(life.alive_cells().len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
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

        Ok(life.alive_cells().len().to_string())
    }
}
