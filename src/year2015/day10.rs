use crate::{Day, Error};

pub struct Day10;
impl Day10 {
    pub fn new() -> Self {
        Self
    }

    fn execute_round(input: &mut String) {
        let mut result = String::new();
        let mut iterator = input.chars();
        let mut curr_char = iterator.next().unwrap();
        let mut count = 1;

        for character in iterator {
            if character == curr_char {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(curr_char);
                curr_char = character;
                count = 1;
            }
        }

        result.push_str(&count.to_string());
        result.push(curr_char);
        *input = result;
    }
}
impl Day for Day10 {
    fn id(&self) -> usize {
        10
    }
    fn title(&self) -> &str {
        "Elves Look, Elves Say"
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().to_string();
        for _ in 0..40 {
            Self::execute_round(&mut input);
        }
        Ok(input.chars().count().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut input = input.trim().to_string();
        for _ in 0..50 {
            Self::execute_round(&mut input);
        }
        Ok(input.chars().count().to_string())
    }
}
