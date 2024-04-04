use crate::shared::structures::Day;

pub fn day_01() -> Day {
    Day::new(1, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let sequence = parse_input(input);
    let mut sum: usize = 0;

    for (i, num) in sequence.iter().enumerate() {
        if num == sequence.get(i + 1).unwrap_or(&sequence[0]) {
            sum += *num as usize;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let sequence = parse_input(input);
    let halfway = sequence.len() >> 1;
    let mut sum: usize = 0;

    for (i, num) in sequence.iter().enumerate() {
        if *num == sequence[(i + halfway) % sequence.len()] {
            sum += *num as usize;
        }
    }

    sum.to_string()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}
