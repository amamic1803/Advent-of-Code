use crate::shared::structures::Day;

pub fn day_18() -> Day {
    Day::new(18, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    count_safe_tiles(ROWS1, parse_input(input)).to_string()
}

fn part2(input: &str) -> String {
    count_safe_tiles(ROWS2, parse_input(input)).to_string()
}

const TRAP_PATTERNS: [[bool; 3]; 4] = [[true, true, false], [false, true, true], [true, false, false], [false, false, true]];
const ROWS1: usize = 40;
const ROWS2: usize = 400_000;

fn count_safe_tiles(rows: usize, mut row: Vec<bool>) -> usize {
    let mut safe_count = row.iter().filter(|&&b| !b).count();
    let mut next_row = vec![false; row.len()];

    for _ in 1..rows {
        for (i, cell) in next_row.iter_mut().enumerate() {
            let mut pattern = [false; 3];
            if let Some(pos) = i.checked_sub(1) {
                if let Some(value) = row.get(pos) {
                    pattern[0] = *value;
                }
            }
            pattern[1] = row[i];
            if let Some(value) = row.get(i + 1) {
                pattern[2] = *value;
            }

            *cell = TRAP_PATTERNS.contains(&pattern);
        }
        (row, next_row) = (next_row, row);
        safe_count += row.iter().filter(|&&b| !b).count();
    }

    safe_count
}

fn parse_input(input: &str) -> Vec<bool> {
    input.trim().chars().map(|c| c == '^').collect()
}
