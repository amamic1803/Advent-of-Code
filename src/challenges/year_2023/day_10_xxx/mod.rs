use crate::shared::structures::Day;

pub fn day_10() -> Day {
    Day::new(
        10,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


use std::collections::HashSet;
use itertools::Itertools;


fn part1(input: &str) -> String {
    let field = parse_input(input);


    // loop will have even number of pipes so we can divide by 2
    // (that is because for every move, up/down/left/right we must do its opposite to get back to S)
    (find_loop(&field).len() / 2).to_string()
}

fn part2(input: &str) -> String {
    let input= ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    let field = parse_input(input);
    let loop_tiles = find_loop(&field);

    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if loop_tiles.contains(&(i, j)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut row_info = vec![Vec::new(); field.len()];
    for (row, col) in loop_tiles.iter() {
        row_info[*row].push(*col);
    }
    for row in row_info.iter_mut() {
        row.sort();
    }

    let mut area = 0;

    for (x, row) in row_info.iter().enumerate() {
        if row.len() > 1 {
            for (i, j) in row.into_iter().tuple_windows().step_by(2) {
                if j - i > 1 {
                    println!("{}=> {}-{}", x, i, j);
                    area += j - i - 1;
                }
            }
        }
    }

    area.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start(field: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found")
}

fn find_adjacent_pipes(pipe: (usize, usize), field: &[Vec<char>]) -> [(usize, usize); 2] {
    match field[pipe.0][pipe.1] {
        'S' => {
            let mut found = 0;
            let mut result = [(0, 0); 2];

            if pipe.0 > 0 && ['|', 'F', '7'].contains(&field[pipe.0 - 1][pipe.1]) {
                result[found] = (pipe.0 - 1, pipe.1);
                found += 1;
            }

            if pipe.0 < field.len() - 1 && ['|', 'J', 'L'].contains(&field[pipe.0 + 1][pipe.1]) {
                result[found] = (pipe.0 + 1, pipe.1);
                found += 1;
            }

            if pipe.1 > 0 && ['-', 'L', 'F'].contains(&field[pipe.0][pipe.1 - 1]) {
                result[found] = (pipe.0, pipe.1 - 1);
                found += 1;
            }

            if pipe.1 < field[0].len() - 1 && ['-', '7', 'J'].contains(&field[pipe.0][pipe.1 + 1]) {
                result[found] = (pipe.0, pipe.1 + 1);
            }

            result
        },
        '|' => [(pipe.0 - 1, pipe.1), (pipe.0 + 1, pipe.1)],
        '-' => [(pipe.0, pipe.1 - 1), (pipe.0, pipe.1 + 1)],
        'L' => [(pipe.0, pipe.1 + 1), (pipe.0 - 1, pipe.1)],
        'J' => [(pipe.0, pipe.1 - 1), (pipe.0 - 1, pipe.1)],
        'F' => [(pipe.0, pipe.1 + 1), (pipe.0 + 1, pipe.1)],
        '7' => [(pipe.0, pipe.1 - 1), (pipe.0 + 1, pipe.1)],
        _ => panic!("Invalid pipe!"),
    }
}

fn find_loop(field: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();

    let mut current = find_start(field);
    let mut next = find_adjacent_pipes(current, field);
    visited.insert(current);

    while !(visited.contains(&next[0]) && visited.contains(&next[1])) {
        if visited.contains(&next[0]) {
            current = next[1];
            visited.insert(current);
            next = find_adjacent_pipes(current, field);
        } else {
            current = next[0];
            visited.insert(current);
            next = find_adjacent_pipes(current, field);
        }
    }

    visited
}
