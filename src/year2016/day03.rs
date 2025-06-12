use crate::{Day, Error};

day!(Day03, 3, "Squares With Three Sides");

impl Day03 {
    fn count_triangles(triangles: Vec<[usize; 3]>) -> usize {
        let mut possible_triangles: usize = 0;

        for triangle in triangles {
            if Self::possible_triangle(triangle) {
                possible_triangles += 1;
            }
        }

        possible_triangles
    }

    fn parse_input(input: &str) -> Vec<[usize; 3]> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut sides = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                sides.sort();
                [sides[0], sides[1], sides[2]]
            })
            .collect()
    }

    fn parse_input_2(input: &str) -> Vec<[usize; 3]> {
        let mut triangles = Vec::new();
        let mut triangle_block = [Vec::new(), Vec::new(), Vec::new()];

        for line in input.trim().lines() {
            for (i, side) in line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).enumerate() {
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

    fn possible_triangle(triangle: [usize; 3]) -> bool {
        triangle[0] + triangle[1] > triangle[2]
    }
}
impl Day for Day03 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let triangles = Self::parse_input(input);
        Ok(Self::count_triangles(triangles).to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let triangles = Self::parse_input_2(input);
        Ok(Self::count_triangles(triangles).to_string())
    }
}
