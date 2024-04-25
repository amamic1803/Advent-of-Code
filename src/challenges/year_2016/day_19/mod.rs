use crate::shared::structures::Day;

pub fn day_19() -> Day {
    Day::new(19, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let elves: usize = match input.trim().parse() {
        Ok(num) => {
            assert!(num > 0, "Number of elves must be greater than 0");
            num
        }
        Err(e) => return format!("Failed to parse input: {}", e),
    };

    let mut circle = Vec::with_capacity(elves);
    for i in 0..(elves - 1) {
        circle.push(i + 1);
    }
    circle.push(0);

    let mut current_elf = 0;
    loop {
        let next_elf = circle[current_elf];
        let new_next_elf = circle[next_elf];
        if new_next_elf == current_elf {
            return (current_elf + 1).to_string();
        } else {
            circle[current_elf] = new_next_elf;
            current_elf = new_next_elf;
        }
    }
}

fn part2(input: &str) -> String {
    let elves: usize = match input.trim().parse() {
        Ok(num) => {
            assert!(num > 0, "Number of elves must be greater than 0");
            num
        }
        Err(e) => return format!("Failed to parse input: {}", e),
    };

    let mut circle = Vec::with_capacity(elves);
    for i in 0..(elves - 1) {
        circle.push(i + 1);
    }
    circle.push(0);

    let mut current_elf = 0;
    let mut circle_len = elves;
    while circle_len > 1 {
        println!("{}", circle_len);
        let mut curr_elf = current_elf;
        let mut next_elf = circle[current_elf];
        for _ in 0..((circle_len >> 1) - 1) {
            (curr_elf, next_elf) = (next_elf, circle[next_elf]);
        }
        circle[curr_elf] = circle[next_elf];
        circle_len -= 1;
        current_elf = circle[current_elf];
    }

    (current_elf + 1).to_string()
}
