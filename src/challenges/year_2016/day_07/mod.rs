use crate::shared::structures::Day;
use regex::Regex;

pub fn day_07() -> Day {
    Day::new(7, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let split_re = Regex::new(r"\[\w*]").unwrap();
    input
        .trim()
        .lines()
        .filter(|line| {
            let is_tls = |elem: &str| {
                for i in 0..(elem.chars().count() - 3) {
                    if elem.chars().nth(i).unwrap() == elem.chars().nth(i + 3).unwrap()
                        && elem.chars().nth(i + 1).unwrap() == elem.chars().nth(i + 2).unwrap()
                        && elem.chars().nth(i).unwrap() != elem.chars().nth(i + 1).unwrap()
                    {
                        return true;
                    }
                }
                false
            };
            let tls_outside = split_re.split(line).any(is_tls);
            let tls_inside = split_re.find_iter(line).any(|mat| is_tls(&line[mat.range()]));

            tls_outside && !tls_inside
        })
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let split_re = Regex::new(r"\[\w*]").unwrap();
    input
        .trim()
        .lines()
        .filter(|line| {
            let supernet = split_re.split(line).filter(|elem| elem.chars().count() >= 3).collect::<Vec<&str>>();
            let hypernet = split_re
                .find_iter(line)
                .map(|mat| &line[(mat.start() + 1)..(mat.end() - 1)])
                .filter(|elem| elem.chars().count() >= 3)
                .collect::<Vec<&str>>();
            for supernet_element in supernet {
                for triple_start in 0..(supernet_element.chars().count() - 2) {
                    let first = supernet_element.chars().nth(triple_start).unwrap();
                    let second = supernet_element.chars().nth(triple_start + 1).unwrap();
                    let third = supernet_element.chars().nth(triple_start + 2).unwrap();
                    if first == third && first != second {
                        let inverted_re = Regex::new(&format!(r"{}{}{}", second, first, second)).unwrap();
                        for hypernet_element in &hypernet {
                            if inverted_re.is_match(hypernet_element) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        })
        .count()
        .to_string()
}
