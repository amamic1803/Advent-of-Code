use crate::{Day, Error};

pub struct Day08;
impl Day08 {
    pub fn new() -> Self {
        Self
    }

    fn encode_len(string: &str) -> (usize, usize) {
        let memory_len = string.chars().count();
        let mut literal_len = memory_len + 2;

        literal_len += string.matches('\\').count();
        literal_len += string.matches('\"').count();

        (literal_len, memory_len)
    }

    fn string_len(string: &str) -> (usize, usize) {
        let literal_len = string.chars().count();
        let mut memory_len = literal_len - 2;

        let mut skip: usize = 0;
        for (i, c) in string.chars().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if c == '\\' {
                if string.chars().nth(i + 1).unwrap() == 'x' {
                    skip = 3;
                    memory_len -= 3;
                } else {
                    skip = 1;
                    memory_len -= 1;
                }
            }
        }

        (literal_len, memory_len)
    }
}
impl Day for Day08 {
    fn id(&self) -> usize {
        8
    }
    fn title(&self) -> &str {
        "Matchsticks"
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut result = 0;
        for line in input.trim().lines() {
            let (literal_len, memory_len) = Self::string_len(line.trim());
            result += literal_len - memory_len;
        }
        Ok(result.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut result: usize = 0;
        for line in input.trim().lines() {
            let (literal_len, memory_len) = Self::encode_len(line.trim());
            result += literal_len - memory_len;
        }
        Ok(result.to_string())
    }
}
