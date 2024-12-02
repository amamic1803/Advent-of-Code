use crate::shared::structures::Day;
use std::ops::RangeInclusive;

pub fn day_02() -> Day {
    Day::new(2, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

const DIFFERENCE_BOUNDS: RangeInclusive<u32> = 1..=3;

fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_safe(line.split_whitespace().map(|x| x.parse::<u32>().unwrap())))
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut safe_count = 0;
    let mut line_elements = Vec::new();
    for line in input.lines() {
        line_elements.clear();
        line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).for_each(|n| line_elements.push(n));

        for i in 0..line_elements.len() {
            let removed_element = line_elements.remove(i);
            if is_safe(line_elements.iter().copied()) {
                safe_count += 1;
                break;
            }
            line_elements.insert(i, removed_element);
        }
    }
    safe_count.to_string()
}

fn is_safe<T: IntoIterator<Item = u32>>(readings: T) -> bool {
    let mut readings = readings.into_iter();
    let mut last_value = if let Some(last_value) = readings.next() {
        last_value
    } else {
        return false;
    };
    let tmp = if let Some(tmp) = readings.next() {
        tmp
    } else {
        return false;
    };
    let ascending = last_value < tmp;
    if !DIFFERENCE_BOUNDS.contains(&last_value.abs_diff(tmp)) {
        return false;
    }
    last_value = tmp;
    for next_value in readings {
        if (!ascending && next_value > last_value) || (ascending && next_value < last_value) || !DIFFERENCE_BOUNDS.contains(&last_value.abs_diff(next_value)) {
            return false;
        }
        last_value = next_value;
    }
    true
}
