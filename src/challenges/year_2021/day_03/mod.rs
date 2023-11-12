use crate::shared::structures::Day;

pub fn day_03() -> Day {
    Day::new(
        3,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let data = parse_input(input);
    let mut gamma: u32 = 0;

    // for each column, count the number of 1s and 0s
    // only one variable (count) is used
    // for true it is incremented, for false it is decremented
    // if it is positive, the bit is 1, otherwise it is 0
    for i in 0..data[0].len() {
        let mut count = 0;
        for data_row in &data {
            if data_row[i] {
                count += 1;
            } else {
                count -= 1;
            }
        }

        // add the bit to the gamma value (to the right)
        gamma <<= 1;
        if count > 0 {
            gamma |= 1;
        }
    }

    // gamma bits need to be inverted
    // simple bitwise NOT does not work (since only the number of bits used should be inverted, but NOT inverts all bits)
    // instead, the gamma value is XORed with a number with the same number of bits as gamma, but all 1ss
    // first, create the inverse gamma value
    let mut inverse_gamma: u32 = 0;
    for _ in 0..data[0].len() {
        inverse_gamma <<= 1;
        inverse_gamma |= 1;
    }

    (gamma * (gamma ^ inverse_gamma)).to_string()
}

fn part2(input: &str) -> String {
    let data = parse_input(input);

}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    // store each line as a vector of bools
    // true is a 1, false is a 0

    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '1' => true,
                    '0' => false,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}