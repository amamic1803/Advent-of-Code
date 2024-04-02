use crate::shared::structures::Day;

pub fn day_05() -> Day {
    Day::new(
        5,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let mut instructions = parse_input(input);
    let mut index = 0;
    let mut steps = 0;

    while index >= 0 && index < instructions.len() as i32 {
        let jump = instructions[index as usize];
        instructions[index as usize] += 1;
        index += jump;
        steps += 1;
    }

    steps.to_string()
}

fn part2(input: &str) -> String {
    let mut instructions = parse_input(input);
    let mut index = 0;
    let mut steps = 0;

    while index >= 0 && index < instructions.len() as i32 {
        let jump = instructions[index as usize];
        if jump >= 3 {
            instructions[index as usize] -= 1;
        } else {
            instructions[index as usize] += 1;
        }
        index += jump;
        steps += 1;
    }

    steps.to_string()
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect()
}
