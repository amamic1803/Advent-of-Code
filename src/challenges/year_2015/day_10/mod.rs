use crate::challenges::Day;

pub(crate) fn day_10() -> Day {
    Day::new (
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    let mut input = input.trim().to_string();
    for _ in 0..40 {
        execute_round(&mut input);
    }
    println!("{}", input.chars().count());
}

fn part2(input: &str) {
    let mut input = input.trim().to_string();
    for _ in 0..50 {
        execute_round(&mut input);
    }
    println!("{}", input.chars().count());
}

fn execute_round(input: &mut String) {
    let mut result: String = String::new();
    let mut iterator = input.chars();
    let mut curr_char: char = iterator.next().unwrap();
    let mut count: usize = 1;

    for character in iterator {
        if character == curr_char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(curr_char);
            curr_char = character;
            count = 1;
        }
    }

    result.push_str(&count.to_string());
    result.push(curr_char);
    *input = result;
}