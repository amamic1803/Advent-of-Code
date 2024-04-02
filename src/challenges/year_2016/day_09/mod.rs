use crate::shared::structures::Day;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn day_09() -> Day {
    Day::new(
        9,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let mut file = String::from(input.trim());
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut position = 0;

    while !file.is_empty() {
        let re_match = match re.find_at(&file, position) {
            Some(caps) => caps,
            None => break,
        };
        let start = re_match.start();
        let range = re_match.range();

        let len = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let times = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        file.replace_range(range, "");
        let compressed_str = file.get(start..(start + len)).unwrap();
        file.insert_str(start, &compressed_str.repeat(times - 1));

        position = start + len * times;
    }

    file.chars().count().to_string()
}

fn part2(input: &str) -> String {
    decompress(input.trim()).to_string()
}

fn decompress(mut current_string: &str) -> u64 {
    // assuming that no marker constructs a new marker while decompressing
    // seems to work for the input, I guess the input is intentionally constructed like this

    let mut file_len: u64 = 0;
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((\d+)x(\d+)\)").unwrap());

    while !current_string.is_empty() {
        let re_match = match RE.find(current_string) {
            Some(caps) => caps,
            None => {
                file_len += current_string.chars().count() as u64;
                break;
            }
        };
        let start = re_match.start();
        let end = re_match.end();

        let len = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let times = re_match
            .as_str()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split('x')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        file_len += start as u64;
        file_len += decompress(&current_string[end..(end + len)]) * times as u64;

        current_string = &current_string[(end + len)..];
    }

    file_len
}
