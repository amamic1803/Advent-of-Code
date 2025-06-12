use crate::{Day, Error};

day!(Day04, 4, "High-Entropy Passphrases");

impl Day for Day04 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .trim()
            .lines()
            .filter(|line| {
                let mut words = line.split_whitespace().collect::<Vec<_>>();
                let original_len = words.len();
                words.sort();
                words.dedup();
                original_len == words.len()
            })
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(input
            .trim()
            .lines()
            .filter(|line| {
                let mut words = line
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars().collect::<Vec<_>>();
                        chars.sort();
                        chars
                    })
                    .collect::<Vec<_>>();
                let original_len = words.len();
                words.sort();
                words.dedup();
                original_len == words.len()
            })
            .count()
            .to_string())
    }
}
