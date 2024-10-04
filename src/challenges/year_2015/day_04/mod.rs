use crate::shared::structures::Day;
use md5::{Digest, Md5};
use std::fmt::Write;

pub fn day_04() -> Day {
    Day::new(4, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut input = input.trim().to_string();
    let input_len = input.len();
    let mut hasher = Md5::new();

    for i in 0.. {
        input.truncate(input_len);
        write!(&mut input, "{}", i).unwrap();
        hasher.update(&input);
        let hash = hasher.finalize_reset();
        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
            return i.to_string();
        }
    }

    unreachable!()
}

fn part2(input: &str) -> String {
    let mut input = input.trim().to_string();
    let input_len = input.len();
    let mut hasher = Md5::new();

    for i in 0.. {
        input.truncate(input_len);
        write!(&mut input, "{}", i).unwrap();
        hasher.update(&input);
        let hash = hasher.finalize_reset();
        if hash.starts_with(&[0, 0, 0]) {
            return i.to_string();
        }
    }

    unreachable!()
}
