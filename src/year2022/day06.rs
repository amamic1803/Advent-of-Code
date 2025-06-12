use crate::{Day, Error};

day!(Day06, 6, "Tuning Trouble");

impl Day06 {
    fn start_of_packet(inp: &str) -> bool {
        for x in inp.chars() {
            let mut counted = 0;
            for y in inp.chars() {
                if x == y {
                    counted += 1;
                }
            }
            if counted > 1 {
                return false;
            }
        }
        true
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
        let mut characters = 4;
        while !Self::start_of_packet(&input[(characters - 4)..characters]) {
            characters += 1;
        }
        Ok(characters.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut characters = 14;
        while !Self::start_of_packet(&input[(characters - 14)..characters]) {
            characters += 1;
        }
        Ok(characters.to_string())
    }
}
