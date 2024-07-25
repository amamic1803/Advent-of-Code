use crate::shared::structures::Day;
use itertools::Itertools;

pub fn day_19() -> Day {
    Day::new(19, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let mut grid = Grid::new(input);
    grid.process();
    grid.letters
}

fn part2(input: &str) -> String {
    let mut grid = Grid::new(input);
    grid.process();
    grid.steps.to_string()
}

struct Grid {
    tubes: Vec<Vec<char>>,
    letters: String,
    steps: u32,
}
impl Grid {
    fn new(input: &str) -> Self {
        Self {
            tubes: input.lines().map(|line| line.chars().collect_vec()).collect_vec(),
            letters: String::new(),
            steps: 0,
        }
    }

    fn process(&mut self) {
        let mut position = [0, self.tubes[0].iter().position(|&c| c == '|').unwrap()];
        let mut direction = 2; // 0: up, 1: right, 2: down, 3: left

        while self.tubes[position[0]][position[1]] != ' ' {
            self.steps += 1;
            if self.tubes[position[0]][position[1]].is_alphabetic() {
                self.letters.push(self.tubes[position[0]][position[1]]);
            } else if self.tubes[position[0]][position[1]] == '+' {
                let mut connections = [false; 4]; // up, right, down, left

                // up
                if position[0] > 0 && self.tubes[position[0] - 1][position[1]] == '|' {
                    connections[0] = true;
                }
                // right
                if position[1] < self.tubes[position[0]].len() - 1 && self.tubes[position[0]][position[1] + 1] == '-' {
                    connections[1] = true;
                }
                // down
                if position[0] < self.tubes.len() - 1 && self.tubes[position[0] + 1][position[1]] == '|' {
                    connections[2] = true;
                }
                // left
                if position[1] > 0 && self.tubes[position[0]][position[1] - 1] == '-' {
                    connections[3] = true;
                }

                let next_direction = connections
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &b)| if b && i != (direction + 2) % 4 { Some(i) } else { None })
                    .next()
                    .unwrap();

                direction = next_direction;
            }
            match direction {
                0 => {
                    if position[0] > 0 {
                        position[0] -= 1
                    } else {
                        break;
                    }
                }
                1 => {
                    if position[1] < self.tubes[position[0]].len() - 1 {
                        position[1] += 1
                    } else {
                        break;
                    }
                }
                2 => {
                    if position[0] < self.tubes.len() - 1 {
                        position[0] += 1
                    } else {
                        break;
                    }
                }
                3 => {
                    if position[1] > 0 {
                        position[1] -= 1
                    } else {
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
