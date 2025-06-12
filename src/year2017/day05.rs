use crate::{Day, Error};

day!(Day05, 5, "A Maze of Twisty Trampolines, All Alike");

impl Day05 {
    fn parse_input(input: &str) -> Vec<i32> {
        input.trim().lines().map(|line| line.trim().parse::<i32>().unwrap()).collect()
    }
}
impl Day for Day05 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut instructions = Self::parse_input(input);
        let mut index = 0;
        let mut steps = 0;

        while index >= 0 && index < instructions.len() as i32 {
            let jump = instructions[index as usize];
            instructions[index as usize] += 1;
            index += jump;
            steps += 1;
        }

        Ok(steps.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut instructions = Self::parse_input(input);
        let mut index = 0;
        let mut steps = 0;

        while index >= 0 && index < instructions.len() as i32 {
            let jump = instructions[index as usize];
            if jump >= 3 {
                instructions[index as usize] -= 1;
            } else {
                instructions[index as usize] += 1;
            }
            index += jump;
            steps += 1;
        }

        Ok(steps.to_string())
    }
}
