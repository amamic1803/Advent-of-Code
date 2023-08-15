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
    let mut prev: usize = usize::MAX;
    let mut count: usize = 0;

    for line in input.trim().lines() {
        let num = line.parse::<usize>().unwrap();
        if num > prev {
            count += 1;
        }
        prev = num;
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let input: Vec<usize> = input.trim().lines().map(|line| line.parse::<usize>().unwrap()).collect();
    let mut count: usize = 0;

    for i in 0..(input.len() - 3) {
        if (input[i + 1] + input[i + 2] + input[i + 3]) > (input[i] + input[i + 1] + input[i + 2]) {
            count += 1;
        }
    }

    count.to_string()
}
