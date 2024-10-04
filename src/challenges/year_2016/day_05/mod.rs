use crate::shared::math::HEX_DIGITS;
use crate::shared::structures::Day;
use md5::{Digest, Md5};
use std::fmt::Write;

pub fn day_05() -> Day {
    Day::new(5, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut password = String::with_capacity(8);

    let mut input_str = String::from(input.trim());
    let input_len = input_str.len();
    let mut hasher = Md5::new();

    for i in 0.. {
        input_str.truncate(input_len);
        write!(&mut input_str, "{}", i).unwrap();

        hasher.update(&input_str);
        let hash = hasher.finalize_reset();

        if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4 == 0) {
            write!(&mut password, "{:x}", hash[2] & 0x0f).unwrap();
            if password.len() == 8 {
                break;
            }
        }
    }

    password
}

fn part2(input: &str) -> String {
    let mut password = ['_'; 8];
    let mut i = 0;

    let mut input_str = String::from(input.trim());
    let input_len = input_str.len();
    let mut hasher = Md5::new();

    while password.iter().any(|&c| c == '_') {
        loop {
            input_str.truncate(input_len);
            write!(&mut input_str, "{}", i).unwrap();

            hasher.update(&input_str);
            let hash = hasher.finalize_reset();

            if hash[0] == 0 && hash[1] == 0 && (hash[2] >> 4 == 0) {
                let position = hash[2] & 0x0f;
                let character = HEX_DIGITS[(hash[3] >> 4) as usize];
                if position < 8 && password[position as usize] == '_' {
                    password[position as usize] = character;
                    break;
                }
            }

            i += 1;
        }
    }

    password.iter().collect()
}
