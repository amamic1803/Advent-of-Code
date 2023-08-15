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
    let mut floor: isize = 0;
    for c in input.trim().chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor.to_string()
}

fn part2(input: &str) -> String {
    let mut floor: isize = 0;
    for (i, c) in input.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return format!("{}", i + 1);
        }
    }

    "Not found!".to_string()
}