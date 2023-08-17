use crate::shared::structures::Day;

pub fn day_20() -> Day {
    Day::new (
        20,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let input = input.trim().parse::<usize>().unwrap();
    let mut sieve = vec![0; input / 10];

    for i in 0..sieve.len() {
        let add_amount = i * 10;
        for j in (i..sieve.len()).step_by(i + 1) {
            sieve[j] += add_amount;
        }
        if sieve[i] >= input {
            return (i + 1).to_string();
        }
    }

    "Number not found!".to_string()
}

fn part2(input: &str) -> String {
    let input = input.trim().parse::<usize>().unwrap();
    let mut sieve = vec![0; (input as f64 / 11.0).ceil() as usize];

    for i in 0..sieve.len() {
        let add_amount = i * 11;
        for (loop_counter, j) in (i..sieve.len()).step_by(i + 1).enumerate() {
            if loop_counter >= 50 {
                break;
            }
            sieve[j] += add_amount;
        }
        if sieve[i] >= input {
            return (i + 1).to_string();
        }
    }

    "Number not found!".to_string()
}