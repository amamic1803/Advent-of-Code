use crate::{Day, Error};

day!(Day02, 2, "I Was Told There Would Be No Math");

impl Day02 {
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
        self.id
    }
    fn title(&self) -> &str {
        self.title
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
