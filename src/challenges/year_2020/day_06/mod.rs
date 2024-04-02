use crate::shared::structures::Day;

pub fn day_06() -> Day {
    Day::new(
        6,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let mut group_answers = [false; 26];
    let mut counts_sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            counts_sum += group_answers.iter().filter(|&&x| x).count();
            group_answers = [false; 26];
        } else {
            for c in line.chars() {
                group_answers[(c as u32 - 'a' as u32) as usize] = true;
            }
        }
    }
    counts_sum += group_answers.iter().filter(|&&x| x).count();

    counts_sum.to_string()
}

fn part2(input: &str) -> String {
    let mut group_answers = [0; 26];
    let mut group_size = 0;
    let mut counts_sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            counts_sum += group_answers.iter().filter(|&&x| x == group_size).count();
            group_answers = [0; 26];
            group_size = 0;
        } else {
            for c in line.chars() {
                group_answers[(c as u32 - 'a' as u32) as usize] += 1;
            }
            group_size += 1;
        }
    }
    counts_sum += group_answers.iter().filter(|&&x| x == group_size).count();

    counts_sum.to_string()
}
