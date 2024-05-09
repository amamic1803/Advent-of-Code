use crate::shared::structures::Day;

pub fn day_17() -> Day {
    Day::new(17, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

const PART1_LIMIT: usize = 2017;
const PART2_LIMIT: usize = 50_000_000;

fn part1(input: &str) -> String {
    let steps = input.trim().parse::<usize>().unwrap();
    let mut circular_buffer = Vec::with_capacity(PART1_LIMIT + 1);
    circular_buffer.push(0);
    let mut i = 0;
    for n in 1..=PART1_LIMIT {
        i = (i + steps) % circular_buffer.len() + 1;
        circular_buffer.insert(i, n);
    }

    circular_buffer[circular_buffer.iter().position(|&elem| elem == PART1_LIMIT).unwrap() + 1].to_string()
}

fn part2(input: &str) -> String {
    let steps = input.trim().parse::<usize>().unwrap();
    let mut circular_buffer = 1;
    let mut result = 0;
    let mut i = 0;
    for n in 1..=PART2_LIMIT {
        i = (i + steps) % circular_buffer + 1;
        if i == 1 {
            result = n;
        }
        circular_buffer += 1;
    }

    result.to_string()
}
