use crate::shared::structures::Day;

pub fn day_02() -> Day {
    Day::new(
        2,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let mut total_paper = 0;
    let input = parse_input(input);

    for present in input {
        total_paper += wrap_present(present);
    }

    format!("{}", total_paper)
}

fn part2(input: &str) -> String {
    let mut total_ribbon = 0;
    let input = parse_input(input);

    for present in input {
        total_ribbon += ribbon_present(present);
    }

    format!("{}", total_ribbon)
}

fn ribbon_present(present: [usize; 3]) -> usize {
    let mut perimeters: [usize; 3] = [
        (present[0] << 1) + (present[1] << 1),
        (present[1] << 1) + (present[2] << 1),
        (present[2] << 1) + (present[0] << 1),
    ];
    perimeters.sort();
    present[0] * present[1] * present[2] + perimeters[0]
}

fn wrap_present(present: [usize; 3]) -> usize {
    let mut sides = [
        present[0] * present[1],
        present[1] * present[2],
        present[2] * present[0],
    ];
    sides.sort();
    3 * sides[0] + 2 * sides[1] + 2 * sides[2]
}

fn parse_input(input: &str) -> Vec<[usize; 3]> {
    let mut result = Vec::new();
    let mut temp: [usize; 3] = [0; 3];
    for line in input.trim().lines() {
        for (i, value) in line.split('x').enumerate() {
            temp[i] = value.parse().unwrap();
        }
        result.push(temp);
    }
    result
}
