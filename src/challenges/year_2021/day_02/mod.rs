use crate::shared::structures::Day;

pub fn day_02() -> Day {
    Day::new(2, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut forward = 0;
    let mut depth = 0;

    for line in input.trim().lines() {
        let mut iter = line.split_whitespace();
        let command_type = iter.next().unwrap();
        let amount: u64 = iter.next().unwrap().parse().unwrap();

        match command_type {
            "forward" => forward += amount,
            "down" => depth += amount,
            "up" => depth = depth.saturating_sub(amount),
            _ => panic!("Invalid command type"),
        }
    }

    (forward * depth).to_string()
}

fn part2(input: &str) -> String {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.trim().lines() {
        let mut iter = line.split_whitespace();
        let command_type = iter.next().unwrap();
        let amount: i64 = iter.next().unwrap().parse().unwrap();

        match command_type {
            "forward" => {
                forward += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Invalid command type"),
        }
    }

    (forward * depth).to_string()
}
