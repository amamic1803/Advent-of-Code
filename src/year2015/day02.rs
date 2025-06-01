use crate::{Day, Error};

pub struct Day02;
impl Day02 {
    pub fn new() -> Self {
        Self
    }
    fn ribbon_present(present: [usize; 3]) -> usize {
        let mut perimeters: [usize; 3] = [
            (present[0] << 1) + (present[1] << 1),
            (present[1] << 1) + (present[2] << 1),
            (present[2] << 1) + (present[0] << 1),
        ];
        perimeters.sort();
        present[0] * present[1] * present[2] + perimeters[0]
    }
    fn wrap_present(present: [usize; 3]) -> usize {
        let mut sides = [present[0] * present[1], present[1] * present[2], present[2] * present[0]];
        sides.sort();
        3 * sides[0] + 2 * sides[1] + 2 * sides[2]
    }
    fn parse_input(input: &str) -> Vec<[usize; 3]> {
        let mut result = Vec::new();
        let mut temp: [usize; 3] = [0; 3];
        for line in input.trim().lines() {
            for (i, value) in line.split('x').enumerate() {
                temp[i] = value.parse().unwrap();
            }
            result.push(temp);
        }
        result
    }
}
impl Day for Day02 {
    fn id(&self) -> usize {
        2
    }
    fn title(&self) -> &str {
        "I Was Told There Would Be No Math"
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut total_paper = 0;
        let input = Self::parse_input(input);

        for present in input {
            total_paper += Self::wrap_present(present);
        }

        Ok(total_paper.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut total_ribbon = 0;
        let input = Self::parse_input(input);

        for present in input {
            total_ribbon += Self::ribbon_present(present);
        }

        Ok(total_ribbon.to_string())
    }
}
