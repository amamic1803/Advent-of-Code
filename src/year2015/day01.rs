use crate::{Day, Error};

pub struct Day01;
impl Day01 {
    pub fn new() -> Self {
        Self
    }
}
impl Day for Day01 {
    fn id(&self) -> usize {
        1
    }
    fn title(&self) -> &str {
        "Not Quite Lisp"
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
