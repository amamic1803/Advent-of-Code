use crate::shared::structures::Day;

pub fn day_03() -> Day {
    Day::new(3, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

// right, down
const SLOPE_PART1: [usize; 2] = [3, 1];
const SLOPES_PART2: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

fn part1(input: &str) -> String {
    let map = parse_input(input);

    count_trees(SLOPE_PART1, &map).to_string()
}

fn part2(input: &str) -> String {
    let map = parse_input(input);

    SLOPES_PART2.into_iter().map(|slope| count_trees(slope, &map)).product::<u64>().to_string()
}

fn count_trees(slope: [usize; 2], map: &[Vec<bool>]) -> u64 {
    let mut position = [0, 0];
    let mut tree_count = 0;

    while position[0] < map.len() {
        if map[position[0]][position[1]] {
            tree_count += 1;
        }
        position[0] += slope[1];
        position[1] += slope[0];
        position[1] %= map[0].len();
    }

    tree_count
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.trim().lines().map(|line| line.chars().map(|c| c == '#').collect()).collect()
}
