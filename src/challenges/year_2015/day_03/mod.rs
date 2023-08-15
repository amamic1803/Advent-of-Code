use crate::shared::structures::Day;
use std::collections::HashSet;

pub fn day_03() -> Day {
    Day::new (
        3,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((x, y));

    for char in input.trim().chars() {
        match char {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Invalid character"),
        }
        visited.insert((x, y));
    }

    format!("{}", visited.len())
}

fn part2(input: &str) -> String {
    let mut x_santa: isize = 0;
    let mut x_robo: isize = 0;
    let mut y_santa: isize = 0;
    let mut y_robo: isize = 0;
    let mut turn: bool = true;  // true == santa, false == robo
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((x_santa, y_santa));

    for char in input.trim().chars() {
        if turn {
            match char {
                '^' => y_santa += 1,
                'v' => y_santa -= 1,
                '>' => x_santa += 1,
                '<' => x_santa -= 1,
                _ => panic!("Invalid character"),
            }
            visited.insert((x_santa, y_santa));
        } else {
            match char {
                '^' => y_robo += 1,
                'v' => y_robo -= 1,
                '>' => x_robo += 1,
                '<' => x_robo -= 1,
                _ => panic!("Invalid character"),
            }
            visited.insert((x_robo, y_robo));
        }
        turn = !turn;
    }

    format!("{}", visited.len())
}