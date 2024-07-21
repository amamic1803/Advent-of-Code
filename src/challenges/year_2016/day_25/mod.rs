use crate::shared::structures::Day;

pub fn day_25() -> Day {
    Day::new(25, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    // input code is meant to be analysed by hand
    // what code does is take a value, add some constant to it (found in the input),
    // and output its binary digits starting from the least significant bit
    // so to get alternating 0s and 1s,
    // the number must be in the form 1010...1010

    // since the value is positive integer, it needs
    // we need to find the first number of the form 1010...1010
    // greater than the constant and subtract the constant from it to get the solution

    let constant = input
        .lines()
        .skip(1)
        .take(2)
        .map(|line| line.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap())
        .product::<usize>();

    let mut number = 0;
    while number < constant {
        number <<= 2;
        number |= 0b10;
    }

    (number - constant).to_string()
}

fn part2(_input: &str) -> String {
    String::from("Advent of Code 2016 solved!")
}
