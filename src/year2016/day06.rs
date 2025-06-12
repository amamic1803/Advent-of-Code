use crate::{Day, Error};
use std::collections::HashMap;

day!(Day06, 6, "Signals and Noise");

impl Day06 {
    fn solve(input: &str, minimum: bool) -> String {
        let message_len = input.trim().lines().next().unwrap().trim().chars().count();
        let mut message: Vec<HashMap<char, usize>> = Vec::with_capacity(message_len);
        for _ in 0..message_len {
            message.push(HashMap::new());
        }

        for line in input.trim().lines() {
            for (i, c) in line.chars().enumerate() {
                let count = message[i].entry(c).or_insert(0);
                *count += 1;
            }
        }

        let mut result = String::new();

        for val in message {
            result.push(if minimum {
                *val.iter().min_by_key(|&(_, v)| v).unwrap().0
            } else {
                *val.iter().max_by_key(|&(_, v)| v).unwrap().0
            });
        }

        result
    }
}
impl Day for Day06 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Self::solve(input, false))
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Self::solve(input, true))
    }
}
