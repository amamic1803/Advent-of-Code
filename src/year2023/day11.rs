use crate::{Day, Error};

day!(Day11, 11, "Cosmic Expansion");

impl Day11 {
    const EXPANSION_1: u64 = 2;
    const EXPANSION_2: u64 = 1_000_000;

    fn parse_input(input: &str) -> Vec<Vec<bool>> {
        input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Invalid input"),
                    })
                    .collect()
            })
            .collect()
    }

    fn sum_of_distances(map: &[Vec<bool>], expansion: u64) -> u64 {
        let mut sum = 0;
        let mut stars = Vec::new();
        let mut empty_rows = Vec::new();
        let mut empty_cols = Vec::new();

        // find stars and empty rows
        for (i, row) in map.iter().enumerate() {
            if row.iter().all(|&x| !x) {
                empty_rows.push(i);
            } else {
                for (j, col) in row.iter().enumerate() {
                    if *col {
                        stars.push((i, j));
                    }
                }
            }
        }

        // find empty cols
        for j in 0..map[0].len() {
            if map.iter().all(|row| !row[j]) {
                empty_cols.push(j);
            }
        }

        // calculate the distance between each pair of stars
        for i in 0..(stars.len() - 1) {
            for j in (i + 1)..stars.len() {
                let (mut x1, mut y1) = stars[i];
                let (mut x2, mut y2) = stars[j];

                if x1 > x2 {
                    (x1, x2) = (x2, x1);
                }
                if y1 > y2 {
                    (y1, y2) = (y2, y1);
                }

                let mut delta_x = x1.abs_diff(x2) as u64;
                let mut delta_y = y1.abs_diff(y2) as u64;

                // add expanded rows
                for empty_row in &empty_rows {
                    if *empty_row > x1 && *empty_row < x2 {
                        delta_x += expansion - 1; // -1 because we already counted the row once
                    }
                }

                // add expanded cols
                for empty_col in &empty_cols {
                    if *empty_col > y1 && *empty_col < y2 {
                        delta_y += expansion - 1; // -1 because we already counted the col once
                    }
                }

                sum += delta_x + delta_y;
            }
        }

        sum
    }
}
impl Day for Day11 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let sky_map = Self::parse_input(input);
        Ok(Self::sum_of_distances(&sky_map, Self::EXPANSION_1).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let sky_map = Self::parse_input(input);
        Ok(Self::sum_of_distances(&sky_map, Self::EXPANSION_2).to_string())
    }
}
