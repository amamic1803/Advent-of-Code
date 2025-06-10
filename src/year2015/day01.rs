use crate::{Day, Error};

day!(Day01, 1, "Not Quite Lisp");

impl Day for Day01 {
    fn id(&self) -> usize {
        self.id
    }
    fn title(&self) -> &str {
        self.title
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut floor: isize = 0;
        for c in input.trim().chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }
        Ok(floor.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut floor: isize = 0;
        for (i, c) in input.trim().chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                return Ok((i + 1).to_string());
            }
        }
        Err(Error::NoSolutionFound)
    }
}
