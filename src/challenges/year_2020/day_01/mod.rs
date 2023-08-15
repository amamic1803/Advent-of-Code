use crate::shared::structures::Day;

pub fn day_01() -> Day {
    Day::new(
        1,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let set = store_input_in_set(input);
    for num in &set {
        let diff = 2020 - num;
        if set.contains(&diff) {
            return format!("{}", num * diff);
        }
    }
    String::from("No solution found")
}

fn part2(input: &str) -> String {
    let set = store_input_in_set(input);
    for (i, num1) in set.iter().enumerate() {
        let diff1 = 2020 - num1;
        for (j, num2) in set.iter().enumerate() {
            if i == j || num2 > &diff1{
                continue;
            }
            let diff2 = diff1 - num2;
            if set.contains(&diff2) {
                return format!("{}", num1 * num2 * diff2);
            }
        }
    }

    String::from("No solution found")
}

fn store_input_in_set(input: &str) -> Vec<usize> {
    let mut set: Vec<usize> = Vec::new();
    for line in input.trim().lines() {
        set.push(line.parse::<usize>().unwrap());
    }
    set
}
