use crate::{Day, Error};

day!(Day09, 9, "Mirage Maintenance");

impl Day09 {
    fn parse_input(input: &str) -> Vec<Vec<i64>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn predict_next(data: &[i64]) -> i64 {
        let mut last_values = Vec::new();
        let mut working_values = data.to_vec();

        while working_values.iter().any(|&val| val != 0) {
            last_values.push(working_values[working_values.len() - 1]);

            for i in 0..(working_values.len() - 1) {
                working_values[i] = working_values[i + 1] - working_values[i];
            }

            working_values.pop();
        }

        last_values.into_iter().sum()
    }

    fn predict_prev(data: &[i64]) -> i64 {
        let mut first_values = Vec::new();
        let mut working_values = data.to_vec();

        while working_values.iter().any(|&val| val != 0) {
            first_values.push(working_values[0]);

            for i in 0..(working_values.len() - 1) {
                working_values[i] = working_values[i + 1] - working_values[i];
            }

            working_values.pop();
        }

        let mut result = 0;
        first_values
            .into_iter()
            .rev()
            .for_each(|val| result = val - result);
        result
    }
}
impl Day for Day09 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let data = Self::parse_input(input);
        Ok(data
            .into_iter()
            .map(|row| Self::predict_next(&row))
            .sum::<i64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let data = Self::parse_input(input);
        Ok(data
            .into_iter()
            .map(|row| Self::predict_prev(&row))
            .sum::<i64>()
            .to_string())
    }
}
