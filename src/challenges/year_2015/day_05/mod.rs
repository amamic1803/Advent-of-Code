use crate::shared::structures::Day;

pub fn day_05() -> Day {
    Day::new (
        5,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    format!("{}", count_nice_strings(input, nice_str))
}

fn part2(input: &str) -> String {
    format!("{}", count_nice_strings(input, nice_str_2))
}

fn count_nice_strings(input: &str, nice_str: fn(&str) -> bool) -> usize {
    let mut nice_strings = 0;
    for line in input.trim().lines() {
        if nice_str(line.trim()) {
            nice_strings += 1;
        }
    }
    nice_strings
}

#[allow(unused_assignments)]
fn nice_str(string: &str) -> bool {
    let vowel_count = string.chars().filter(|c| "aeiou".contains(*c)).count();

    let mut last_char= ' ';
    let mut double_letter = false;
    for (i, c) in string.chars().enumerate() {
        if i == 0 {
            last_char = c;
        } else if c == last_char {
            double_letter = true;
            break;
        }
        last_char = c;
    }

    let disallowed_substrings: bool =
        string.contains("ab") ||
        string.contains("cd") ||
        string.contains("pq") ||
        string.contains("xy");

    vowel_count >= 3 && double_letter && !disallowed_substrings
}

fn nice_str_2(string: &str) -> bool {
    let string: Vec<char> = string.chars().collect();

    let mut double_pair = false;
    if string.len() < 4 {
        return false;
    }
    'outer: for i in 0..(string.len() - 3) {
        for j in (i + 2)..(string.len() - 1) {
            if string[i] == string[j] && string[i + 1] == string[j + 1] {
                double_pair = true;
                break 'outer;
            }
        }
    }

    let mut letter_between = false;
    for i in 1..(string.len() - 1) {
        if string[i - 1] == string[i + 1] {
            letter_between = true;
            break;
        }
    }

    double_pair && letter_between
}