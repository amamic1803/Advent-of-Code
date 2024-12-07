use crate::shared::structures::Day;

pub fn day_06() -> Day {
    Day::new(6, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let (mut location, mut grid) = parse_input(input);

    loop {
        match location.direction {
            Direction::Up => {
                if location.i == 0 {
                    break;
                }
                if grid[location.i - 1][location.j] == '#' {
                    location.direction = Direction::Right;
                } else {
                    location.i -= 1;
                    grid[location.i][location.j] = 'X';
                }
            }
            Direction::Down => {
                if location.i == grid.len() - 1 {
                    break;
                }
                if grid[location.i + 1][location.j] == '#' {
                    location.direction = Direction::Left;
                } else {
                    location.i += 1;
                    grid[location.i][location.j] = 'X';
                }
            }
            Direction::Left => {
                if location.j == 0 {
                    break;
                }
                if grid[location.i][location.j - 1] == '#' {
                    location.direction = Direction::Up;
                } else {
                    location.j -= 1;
                    grid[location.i][location.j] = 'X';
                }
            }
            Direction::Right => {
                if location.j == grid[0].len() - 1 {
                    break;
                }
                if grid[location.i][location.j + 1] == '#' {
                    location.direction = Direction::Down;
                } else {
                    location.j += 1;
                    grid[location.i][location.j] = 'X';
                }
            }
        }
    }

    grid.into_iter().flat_map(|row| row.into_iter()).filter(|&c| c == 'X').count().to_string()
}

fn part2(_input: &str) -> String {
    String::new()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Location {
    i: usize,
    j: usize,
    direction: Direction,
}

fn parse_input(input: &str) -> (Location, Vec<Vec<char>>) {
    let mut grid = Vec::new();
    let mut location = Location {
        i: 0,
        j: 0,
        direction: Direction::Up,
    };

    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' | '.' => row.push(c),
                '^' => {
                    location.i = i;
                    location.j = j;
                    row.push('X');
                }
                'v' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Down;
                    row.push('X');
                }
                '<' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Left;
                    row.push('X');
                }
                '>' => {
                    location.i = i;
                    location.j = j;
                    location.direction = Direction::Right;
                    row.push('X');
                }
                _ => panic!("Invalid character in input"),
            }
        }
        grid.push(row);
    }

    (location, grid)
}
