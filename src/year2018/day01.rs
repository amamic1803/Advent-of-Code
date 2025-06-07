use crate::{Day, Error};
use std::collections::HashSet;

pub struct Day01;
impl Day01 {
    pub fn new() -> Self {
        Self
    }

    fn parse_input(input: &str) -> Vec<isize> {
        input.trim().lines().map(|line| line.parse::<isize>().unwrap()).collect()
    }
}
impl Day for Day01 {
    fn id(&self) -> usize {
        1
    }

    fn title(&self) -> &str {
        "Chronal Calibration"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let input = Self::parse_input(input);
        Ok(input.iter().sum::<isize>().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let input = Self::parse_input(input);
        let mut i = 0;
        let mut encountered_frequencies: HashSet<isize> = HashSet::new();
        let mut frequency = 0;

        loop {
            frequency += input[i];
            if encountered_frequencies.insert(frequency) {
                i = (i + 1) % input.len();
            } else {
                return Ok(frequency.to_string());
            }
        }
    }
}
