use crate::shared::structures::Day;

pub fn day_03() -> Day {
    Day::new(
        3,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let triangles = parse_input(input);
    count_triangles(triangles).to_string()
}

fn part2(input: &str) -> String {
    let triangles = parse_input_2(input);
    count_triangles(triangles).to_string()
}

type Triangle = [usize; 3];

fn count_triangles(triangles: Vec<Triangle>) -> usize {
    let mut possible_triangles: usize = 0;

    for triangle in triangles {
        if possible_triangle(triangle) {
            possible_triangles += 1;
        }
    }

    possible_triangles
}

fn parse_input(input: &str) -> Vec<Triangle> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            sides.sort();
            [sides[0], sides[1], sides[2]]
        })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut triangle_block: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new()];

    for line in input.trim().lines() {
        for (i, side) in line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .enumerate()
        {
            triangle_block[i].push(side);
        }

        if triangle_block[0].len() == 3 {
            for triangle in triangle_block.iter_mut() {
                triangle.sort();
                triangles.push([triangle[0], triangle[1], triangle[2]]);
                triangle.clear();
            }
        }
    }

    triangles
}

fn possible_triangle(triangle: Triangle) -> bool {
    triangle[0] + triangle[1] > triangle[2]
}
