use crate::{Day, Error};

day!(Day03, 3, "Rucksack Reorganization");

impl Day03 {
    const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn letter_value(letter: char) -> u64 {
        let mut value: u64 = 1;
        for x in Self::ALPHABET.chars() {
            if letter == x {
                break;
            }
            value += 1;
        }
        value
    }
}
impl Day for Day03 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut result: u64 = 0;
        for line in input.lines() {
            if !line.is_empty() {
                let (first, last) = line.split_at(line.chars().count() / 2);
                for character in first.chars() {
                    if last.contains(character) {
                        result += Self::letter_value(character);
                        break;
                    }
                }
            }
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut result: u64 = 0;
        let mut curr_group_members: Vec<&str> = vec![];
        for line in input.lines() {
            if curr_group_members.len() == 3 {
                for character in curr_group_members[0].chars() {
                    if curr_group_members[1].contains(character) && curr_group_members[2].contains(character) {
                        result += Self::letter_value(character);
                        break;
                    }
                }
                curr_group_members = vec![line];
            } else {
                curr_group_members.push(line);
            }
        }
        if curr_group_members.len() == 3 {
            for character in curr_group_members[0].chars() {
                if curr_group_members[1].contains(character) && curr_group_members[2].contains(character) {
                    result += Self::letter_value(character);
                    break;
                }
            }
        }
        Ok(result.to_string())
    }
}
