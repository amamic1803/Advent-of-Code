use crate::{Day, Error};
use std::collections::HashSet;

pub struct Day03;
impl Day03 {
    pub fn new() -> Self {
        Self
    }
}
impl Day for Day03 {
    fn id(&self) -> usize {
        3
    }
    fn title(&self) -> &str {
        "Perfectly Spherical Houses in a Vacuum"
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        visited.insert((x, y));

        for char in input.trim().chars() {
            match char {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => panic!("Invalid character"),
            }
            visited.insert((x, y));
        }

        Ok(visited.len().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut x_santa: isize = 0;
        let mut x_robo: isize = 0;
        let mut y_santa: isize = 0;
        let mut y_robo: isize = 0;
        let mut turn: bool = true; // true == santa, false == robo
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        visited.insert((x_santa, y_santa));

        for char in input.trim().chars() {
            if turn {
                match char {
                    '^' => y_santa += 1,
                    'v' => y_santa -= 1,
                    '>' => x_santa += 1,
                    '<' => x_santa -= 1,
                    _ => panic!("Invalid character"),
                }
                visited.insert((x_santa, y_santa));
            } else {
                match char {
                    '^' => y_robo += 1,
                    'v' => y_robo -= 1,
                    '>' => x_robo += 1,
                    '<' => x_robo -= 1,
                    _ => panic!("Invalid character"),
                }
                visited.insert((x_robo, y_robo));
            }
            turn = !turn;
        }

        Ok(visited.len().to_string())
    }
}
