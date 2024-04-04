use crate::shared::structures::Day;

pub fn day_01() -> Day {
    Day::new(1, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut sum: usize = 0;

    for num in input.trim().lines() {
        sum += num.parse::<usize>().unwrap() / 3;
        sum = sum.saturating_sub(2);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum: usize = 0;

    for num in input.trim().lines() {
        let module_fuel = (num.parse::<usize>().unwrap() / 3).saturating_sub(2);
        sum += module_fuel + fuel_for_fuel(module_fuel);
    }

    sum.to_string()
}

fn fuel_for_fuel(mut fuel: usize) -> usize {
    let mut sum: usize = 0;

    while fuel > 2 {
        fuel /= 3;
        fuel = fuel.saturating_sub(2);
        sum += fuel;
    }

    sum
}
