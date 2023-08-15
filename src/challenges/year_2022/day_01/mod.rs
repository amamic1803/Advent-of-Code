use crate::shared::structures::Day;

pub fn day_01() -> Day {
    Day::new(
        1,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let elf_calories = count_calories(input);
    elf_calories[elf_calories.len() - 1].to_string()
}

fn part2(input: &str) -> String {
    let elf_calories = count_calories(input);
    elf_calories[(elf_calories.len() - 3)..].iter().sum::<usize>().to_string()
}

fn count_calories(input: &str) -> Vec<usize> {
    let mut elf_calories: Vec<usize> = vec![];
    let mut curr_calorie: usize = 0;
    let mut elf: bool = false;
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