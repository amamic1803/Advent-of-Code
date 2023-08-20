use crate::shared::structures::Day;
use md5;

pub fn day_05() -> Day {
    Day::new(
        5,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let input = input.trim();
    let mut password = String::new();
    let mut index = 0;
    while password.chars().count() < 8 {
        loop {
            let hash = format!("{:x}", md5::compute(format!("{}{}", input, index)));
            index += 1;
            if hash.starts_with("00000") {
                password.push(hash.chars().nth(5).unwrap());
                break;
            }
        }
    }

    password
}

fn part2(input: &str) -> String {
    let input = input.trim();
    let mut password = ['_'; 8];
    let mut index = 0;

    while password.iter().any(|&c| c == '_') {
        loop {
            let hash = format!("{:x}", md5::compute(format!("{}{}", input, index)));
            index += 1;
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