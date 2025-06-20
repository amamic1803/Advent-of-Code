use crate::{Day, Error};

day!(Day01, 1, "Calorie Counting");

impl Day01 {
    fn count_calories(input: &str) -> Vec<usize> {
        let mut elf_calories = vec![];
        let mut curr_calorie = 0;
        let mut elf = false;
        for line in input.lines() {
            if !line.is_empty() {
                elf = true;
                curr_calorie += line.parse::<usize>().unwrap();
            } else {
                elf_calories.push(curr_calorie);
                curr_calorie = 0;
                elf = false;
            }
        }
        if elf {
            elf_calories.push(curr_calorie);
        }
        elf_calories.sort();
        elf_calories
    }
}
impl Day for Day01 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let elf_calories = Self::count_calories(input);
        Ok(elf_calories[elf_calories.len() - 1].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let elf_calories = Self::count_calories(input);
        Ok(elf_calories[(elf_calories.len() - 3)..]
            .iter()
            .sum::<usize>()
            .to_string())
    }
}
