use crate::shared::structures::Day;

pub fn day_14() -> Day {
    Day::new(
        14,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

use super::day_10::KnotHash;
use std::collections::VecDeque;

const ROWS: usize = 128;

fn part1(input: &str) -> String {
    let grid = generate_grid(input.trim());
    let mut count = 0;

    for row in grid.iter() {
        for cell in row.iter() {
            if *cell {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let mut grid = generate_grid(input.trim());
    let mut region_count = 0;

    let mut queue = VecDeque::new();
    while let Some(used_pos) = find_used(&grid) {
        region_count += 1;
        queue.push_back(used_pos);

        while !queue.is_empty() {
            let working_pos = queue.pop_front().unwrap();
            if grid[working_pos.0][working_pos.1] {
                grid[working_pos.0][working_pos.1] = false;

                if working_pos.0 > 0 && grid[working_pos.0 - 1][working_pos.1] {
                    queue.push_back((working_pos.0 - 1, working_pos.1));
                }
                if working_pos.0 < ROWS - 1 && grid[working_pos.0 + 1][working_pos.1] {
                    queue.push_back((working_pos.0 + 1, working_pos.1));
                }
                if working_pos.1 > 0 && grid[working_pos.0][working_pos.1 - 1] {
                    queue.push_back((working_pos.0, working_pos.1 - 1));
                }
                if working_pos.1 < ROWS - 1 && grid[working_pos.0][working_pos.1 + 1] {
                    queue.push_back((working_pos.0, working_pos.1 + 1));
                }
            }
        }
    }

    region_count.to_string()
}

fn generate_grid(input: &str) -> Vec<Vec<bool>> {
    let input_str = input.trim();
    let mut grid = Vec::with_capacity(ROWS); // 128x128 grid, true = used, false = free

    for i in 0..ROWS {
        let mut knot_hash = KnotHash::new();
        let hash_str = format!("{}-{}", input_str, i);
        let hash = knot_hash.hash(&hash_str);

        let mut row = Vec::with_capacity(ROWS);

        for character in hash.chars().map(|c| c.to_digit(16).unwrap() as u8) {
            let mut temp = character.reverse_bits();
            temp >>= 4;

            for _ in 0..4 {
                row.push((temp & 1) == 1);
                temp >>= 1;
            }
        }

        grid.push(row);
    }

    grid
}

fn find_used(grid: &[Vec<bool>]) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                return Some((i, j));
            }
        }
    }

    None
}
