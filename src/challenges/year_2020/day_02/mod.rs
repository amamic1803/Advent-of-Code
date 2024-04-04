use crate::shared::structures::Day;

pub fn day_02() -> Day {
    Day::new(2, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let passwords = parse_input(input);
    passwords
        .iter()
        .filter(|(min, max, character, password)| valid_password_1(*min, *max, *character, password))
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let passwords = parse_input(input);
    passwords
        .iter()
        .filter(|(min, max, character, password)| valid_password_2(*min, *max, *character, password))
        .count()
        .to_string()
}

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

fn valid_password_1(min: usize, max: usize, character: char, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == character).count();
    count >= min && count <= max
}

fn valid_password_2(min: usize, max: usize, character: char, password: &str) -> bool {
    (password.chars().nth(min - 1).unwrap() == character) ^ (password.chars().nth(max - 1).unwrap() == character)
}
