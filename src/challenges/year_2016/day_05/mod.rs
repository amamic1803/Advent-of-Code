use crate::shared::structures::Day;
use md5;
use std::fmt::Write;

pub fn day_05() -> Day {
    Day::new(5, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut i = 0;

    let mut input_str = String::from(input.trim());
    let input_len = input_str.len();

    let mut hash = String::new();

    while password.chars().count() < 8 {
        while !hash.starts_with("00000") {
            input_str.truncate(input_len);
            write!(&mut input_str, "{}", i).unwrap();

            hash.clear();
            write!(&mut hash, "{:x}", md5::compute(&input_str)).unwrap();

            i += 1;
        }
        password.push(hash.chars().nth(5).unwrap());
        hash.clear();
    }

    password
}

fn part2(input: &str) -> String {
    let mut password = ['_'; 8];
    let mut i = 0;

    let mut input_str = String::from(input.trim());
    let input_len = input_str.len();

    let mut hash = String::new();

    while password.iter().any(|&c| c == '_') {
        loop {
            input_str.truncate(input_len);
            write!(&mut input_str, "{}", i).unwrap();

            hash.clear();
            write!(&mut hash, "{:x}", md5::compute(&input_str)).unwrap();

            i += 1;

            if hash.starts_with("00000") {
                match hash.chars().nth(5).unwrap().to_digit(10) {
                    Some(i) if i < 8 && password[i as usize] == '_' => {
                        password[i as usize] = hash.chars().nth(6).unwrap();
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    password.iter().collect()
}
