use crate::shared::structures::Day;

pub fn day_11() -> Day {
    Day::new(11, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut x = 0i32;
    let mut y_half = 0i32;

    input.trim().split(',').for_each(|direction| process_direction(direction, &mut x, &mut y_half));

    distance(x, y_half).to_string()
}

fn part2(input: &str) -> String {
    let mut max_distance = 0;
    let mut x = 0i32;
    let mut y_half = 0i32;

    for direction in input.trim().split(',') {
        process_direction(direction, &mut x, &mut y_half);
        let distance = distance(x, y_half);
        if distance > max_distance {
            max_distance = distance;
        }
    }

    max_distance.to_string()
}

fn process_direction(direction: &str, x: &mut i32, y_half: &mut i32) {
    match direction {
        "n" => *y_half += 2,
        "ne" => {
            *x += 1;
            *y_half += 1;
        }
        "se" => {
            *x += 1;
            *y_half -= 1;
        }
        "s" => *y_half -= 2,
        "sw" => {
            *x -= 1;
            *y_half -= 1;
        }
        "nw" => {
            *x -= 1;
            *y_half += 1;
        }
        _ => panic!("Invalid direction"),
    }
}
fn distance(x: i32, y_half: i32) -> u32 {
    let x = x.unsigned_abs();
    let y = y_half.unsigned_abs().abs_diff(x);
    assert_eq!(y % 2, 0, "Impossible distance");
    let y = y / 2;
    x + y
}
