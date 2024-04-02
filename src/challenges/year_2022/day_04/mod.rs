use crate::shared::structures::Day;

pub fn day_04() -> Day {
    Day::new(
        4,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    process_input(input).0.to_string()
}

fn part2(input: &str) -> String {
    process_input(input).1.to_string()
}

fn process_input(input: &str) -> (usize, usize) {
    let mut contain: usize = 0;
    let mut overlap: usize = 0;
    let mut elves: Vec<usize>;
    for line in input.trim().lines() {
        elves = vec![];
        for elf in line.split(',') {
            for sectors in elf.split('-') {
                elves.push(sectors.parse::<usize>().unwrap())
            }
        }
        if ((elves[0] <= elves[2]) && (elves[1] >= elves[3]))
            || ((elves[2] <= elves[0]) && (elves[3] >= elves[1]))
        {
            contain += 1;
            overlap += 1;
        } else if ((elves[0] >= elves[2]) && (elves[0] <= elves[3]))
            || ((elves[1] >= elves[2]) && (elves[1] <= elves[3]))
        {
            overlap += 1;
        }
    }

    (contain, overlap)
}
