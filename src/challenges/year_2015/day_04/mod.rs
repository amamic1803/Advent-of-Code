use crate::challenges::Day;
use md5::compute;

pub(crate) fn day_04() -> Day {
    Day::new (
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) {
    find_with_zeros(input, 5);
}

fn part2(input: &str) {
    find_with_zeros(input, 6);
}

fn find_with_zeros(input: &str, zeros: usize) {
    let input = input.trim();
    let mut num: usize = 1;
    let mut hash_str: String = String::new();
    let match_template: String = (0..zeros).map(|_| "0").collect();

    loop {
        hash_str.clear();
        hash_str.push_str(&format!("{:x}", compute(format!("{}{}", input, num))));
        if hash_str.starts_with(&match_template) {
            println!("{}", num);
            break;
        }
        num += 1;
    }
}