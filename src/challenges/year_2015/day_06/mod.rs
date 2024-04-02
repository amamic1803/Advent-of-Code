#![allow(clippy::needless_range_loop)]
use crate::shared::structures::Day;

pub fn day_06() -> Day {
    Day::new(
        6,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let instructions = parse_input(input);
    let mut grid = vec![vec![false; 1000]; 1000];

    for ins in instructions {
        execute_instruction(&mut grid, ins);
    }

    format!("{}", lights_on(&grid))
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let mut grid: Vec<Vec<isize>> = vec![vec![0; 1000]; 1000];

    for ins in instructions {
        execute_instruction2(&mut grid, ins);
    }

    format!("{}", brightness(&grid))
}

type Instruction = (u8, (usize, usize), (usize, usize));

fn execute_instruction(grid: &mut [Vec<bool>], ins: Instruction) {
    for i in ins.1 .0..=ins.2 .0 {
        for j in ins.1 .1..=ins.2 .1 {
            match ins.0 {
                0 => grid[i][j] = false,
                1 => grid[i][j] = true,
                2 => grid[i][j] = !grid[i][j],
                _ => unreachable!("Invalid instruction"),
            }
        }
    }
}

fn execute_instruction2(grid: &mut [Vec<isize>], ins: Instruction) {
    for i in ins.1 .0..=ins.2 .0 {
        for j in ins.1 .1..=ins.2 .1 {
            match ins.0 {
                0 => {
                    if grid[i][j] > 0 {
                        grid[i][j] -= 1;
                    }
                }
                1 => grid[i][j] += 1,
                2 => grid[i][j] += 2,
                _ => unreachable!("Invalid instruction"),
            }
        }
    }
}

fn lights_on(grid: &[Vec<bool>]) -> usize {
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                result += 1;
            }
        }
    }

    result
}

fn brightness(grid: &[Vec<isize>]) -> usize {
    let mut result: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            result += usize::try_from(grid[i][j]).unwrap();
        }
    }

    result
}

fn parse_input(input: &str) -> Vec<Instruction> {
    // (instruction, lower, upper)
    // on = 1, off = 0, toggle = 2

    let mut result: Vec<Instruction> = Vec::new();
    let mut line_vec: Vec<&str>;

    for line in input.trim().lines() {
        line_vec = line.split(' ').collect();

        match line_vec[1] {
            "on" => {
                let mut split = line_vec[2].split(',');
                let mut split2 = line_vec[4].split(',');
                result.push((
                    1,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
            "off" => {
                let mut split = line_vec[2].split(',');
                let mut split2 = line_vec[4].split(',');
                result.push((
                    0,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
            _ => {
                let mut split = line_vec[1].split(',');
                let mut split2 = line_vec[3].split(',');
                result.push((
                    2,
                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        split2.next().unwrap().parse().unwrap(),
                        split2.next().unwrap().parse().unwrap(),
                    ),
                ));
            }
        }
    }

    result
}
