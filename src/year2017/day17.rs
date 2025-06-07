use crate::{Day, Error};

pub struct Day17;
impl Day17 {
    pub fn new() -> Self {
        Self
    }

    const PART1_LIMIT: usize = 2017;
    const PART2_LIMIT: usize = 50_000_000;
}
impl Day for Day17 {
    fn id(&self) -> usize {
        17
    }

    fn title(&self) -> &str {
        "Spinlock"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let steps = input.trim().parse::<usize>().unwrap();
        let mut circular_buffer = Vec::with_capacity(Self::PART1_LIMIT + 1);
        circular_buffer.push(0);
        let mut i = 0;
        for n in 1..=Self::PART1_LIMIT {
            i = (i + steps) % circular_buffer.len() + 1;
            circular_buffer.insert(i, n);
        }

        Ok(circular_buffer[circular_buffer.iter().position(|&elem| elem == Self::PART1_LIMIT).unwrap() + 1].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let steps = input.trim().parse::<usize>().unwrap();
        let mut circular_buffer = 1;
        let mut result = 0;
        let mut i = 0;
        for n in 1..=Self::PART2_LIMIT {
            i = (i + steps) % circular_buffer + 1;
            if i == 1 {
                result = n;
            }
            circular_buffer += 1;
        }

        Ok(result.to_string())
    }
}
