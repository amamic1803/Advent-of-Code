use crate::shared::structures::Day;
use itertools::Itertools;

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
    let mut array = parse_input(input);
    array[1] = 12;
    array[2] = 2;

    simulate(&mut array);

    array[0].to_string()
}

fn part2(input: &str) -> String {
    let wanted_output = 19690720;

    let input_array = parse_input(input);

    for (m, n) in (0..100).cartesian_product(0..100) {
        let mut array = input_array.clone();
        array[1] = m;
        array[2] = n;

        simulate(&mut array);

        if array[0] == wanted_output {
            return (100 * m + n).to_string();
        }
    }

    panic!("No solution found");
}

fn simulate(array: &mut [usize]) {
    for i in (0..(array.len() - 3)).step_by(4) {
        match array[i] {
            1 => {
                let pos1 = array[i + 1];
                let pos2 = array[i + 2];
                let pos3 = array[i + 3];
                array[pos3] = array[pos1] + array[pos2];
            },
            2 => {
                let pos1 = array[i + 1];
                let pos2 = array[i + 2];
                let pos3 = array[i + 3];
                array[pos3] = array[pos1] * array[pos2];
            },
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}