use crate::shared::structures::Day;

pub fn day_09() -> Day {
    Day::new(9, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let stream = remove_cancelled_chars(input);

    let mut stack = Vec::new();
    let mut garbage = false;
    for c in stream.chars() {
        match c {
            '{' => {
                if !garbage {
                    stack.push((c, stack.len() + 1));
                }
            }
            '}' => {
                if !garbage {
                    let (_, score) = stack.pop().unwrap();
                    match stack.last_mut() {
                        Some(last) => last.1 += score,
                        None => return score.to_string(),
                    }
                }
            }
            '<' => garbage = true,
            '>' => garbage = false,
            _ => {}
        }
    }

    panic!("Invalid input!");
}

fn part2(input: &str) -> String {
    let stream = remove_cancelled_chars(input);
    let mut garbage_chars = 0;

    let mut garbage = false;
    for c in stream.chars() {
        match c {
            '<' => {
                if garbage {
                    garbage_chars += 1;
                } else {
                    garbage = true
                }
            }
            '>' => garbage = false,
            _ => {
                if garbage {
                    garbage_chars += 1;
                }
            }
        }
    }

    garbage_chars.to_string()
}

fn remove_cancelled_chars(input: &str) -> String {
    let mut stream = String::with_capacity(input.len());

    let mut input_iter = input.trim().chars();
    while let Some(c) = input_iter.next() {
        if c == '!' {
            input_iter.next();
        } else {
            stream.push(c);
        }
    }

    stream
}
