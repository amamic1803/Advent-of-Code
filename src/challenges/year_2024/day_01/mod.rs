use crate::shared::structures::Day;
use std::collections::HashMap;

pub fn day_01() -> Day {
    Day::new(1, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    input.lines().for_each(|line| {
        let mut line_split = line.split_whitespace();
        list1.push(line_split.next().unwrap().parse::<u32>().unwrap());
        list2.push(line_split.next().unwrap().parse::<u32>().unwrap());
    });
    list1.sort();
    list2.sort();

    list1.into_iter().zip(list2).map(|(a, b)| a.abs_diff(b)).sum::<u32>().to_string()
}

fn part2(input: &str) -> String {
    let mut list1 = Vec::new();
    let mut map2 = HashMap::new();
    input.lines().for_each(|line| {
        let mut line_split = line.split_whitespace();
        list1.push(line_split.next().unwrap().parse::<u32>().unwrap());
        let num2 = line_split.next().unwrap().parse::<u32>().unwrap();
        *map2.entry(num2).or_insert(0) += 1;
    });
    list1.into_iter().map(|n| n * map2.get(&n).unwrap_or(&0)).sum::<u32>().to_string()
}
