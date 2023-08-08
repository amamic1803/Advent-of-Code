use crate::challenges::Day;

pub(crate) fn day_01() -> Day {
    Day::new(
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    let mut floor: isize = 0;
    for c in input.trim().chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("{}", floor);
}

fn part2(input: &str) {
    let mut floor: isize = 0;
    let mut broken: bool = false;
    for (i, c) in input.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            println!("{}", i + 1);
            broken = true;
            break;
        }
    }
    if !broken {
        println!("Not found!");
    }
}