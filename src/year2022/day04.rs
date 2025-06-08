use crate::{Day, Error};

pub struct Day04;
impl Day04 {
    pub fn new() -> Self {
        Self
    }

    fn process_input(input: &str) -> (usize, usize) {
        let mut contain: usize = 0;
        let mut overlap: usize = 0;
        let mut elves: Vec<usize>;
        for line in input.trim().lines() {
            elves = vec![];
            for elf in line.split(',') {
                for sectors in elf.split('-') {
                    elves.push(sectors.parse::<usize>().unwrap())
                }
            }
            if ((elves[0] <= elves[2]) && (elves[1] >= elves[3])) || ((elves[2] <= elves[0]) && (elves[3] >= elves[1])) {
                contain += 1;
                overlap += 1;
            } else if ((elves[0] >= elves[2]) && (elves[0] <= elves[3])) || ((elves[1] >= elves[2]) && (elves[1] <= elves[3])) {
                overlap += 1;
            }
        }

        (contain, overlap)
    }
}
impl Day for Day04 {
    fn id(&self) -> usize {
        4
    }

    fn title(&self) -> &str {
        "Camp Cleanup"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Self::process_input(input).0.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Self::process_input(input).1.to_string())
    }
}
