use crate::{Day, Error};

day!(Day02, 2, "Password Philosophy");

impl Day02 {
    fn parse_input(input: &str) -> Vec<(usize, usize, char, &str)> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut split = line.split(' ');
                let mut range = split.next().unwrap().split('-');
                let min = range.next().unwrap().parse::<usize>().unwrap();
                let max = range.next().unwrap().parse::<usize>().unwrap();
                let character = split.next().unwrap().chars().next().unwrap();
                let password = split.next().unwrap();
                (min, max, character, password)
            })
            .collect()
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
        let passwords = Self::parse_input(input);
        Ok(passwords
            .iter()
            .filter(|(min, max, character, password)| {
                let count = password.chars().filter(|c| c == character).count();
                (*min..=*max).contains(&count)
            })
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let passwords = Self::parse_input(input);
        Ok(passwords
            .iter()
            .filter(|(min, max, character, password)| {
                (password.chars().nth(min - 1).unwrap() == *character)
                    ^ (password.chars().nth(max - 1).unwrap() == *character)
            })
            .count()
            .to_string())
    }
}
