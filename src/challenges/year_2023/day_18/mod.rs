use crate::shared::structures::Day;

pub fn day_18() -> Day {
    Day::new(
        18,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


use itertools::Itertools;


fn part1(input: &str) -> String {
    let points = parse_input1(input);
    calculate_volume(&points).to_string()
}

fn part2(input: &str) -> String {
    let points = parse_input2(input);
    calculate_volume(&points).to_string()
}

fn calculate_volume(points: &[(i64, i64)]) -> i64 {
    let boundary_points = boundary_points(points);
    boundary_points + interior_points(points, boundary_points)
}

fn parse_input1(input: &str) -> Vec<(i64, i64)> {
    let mut points = Vec::new();

    let mut curr_point = (0, 0);
    for line in input.trim().lines() {
        let (ins, val, _) = line.split_whitespace().collect_tuple().unwrap();
        let val = val.parse::<i64>().unwrap();

        curr_point = match ins {
            "R" => (curr_point.0 + val, curr_point.1),
            "L" => (curr_point.0 - val, curr_point.1),
            "U" => (curr_point.0, curr_point.1 + val),
            "D" => (curr_point.0, curr_point.1 - val),
            _ => panic!("Invalid instruction"),
        };

        points.push(curr_point);
    }

    points
}

fn parse_input2(input: &str) -> Vec<(i64, i64)> {
    let mut points = Vec::new();

    let mut curr_point = (0, 0);
    for line in input.trim().lines() {
        let (_, _, hex_color) = line.split_whitespace().collect_tuple().unwrap();
        let hex_color = hex_color.trim_start_matches("(#").trim_end_matches(')');

        let (_, ins, val) = hex_color.rsplitn(3, "").collect_tuple().unwrap();

        let ins = ins.chars().next().unwrap();
        let val = i64::from_str_radix(val, 16).unwrap();

        curr_point = match ins {
            '0' => (curr_point.0 + val, curr_point.1),
            '2' => (curr_point.0 - val, curr_point.1),
            '3' => (curr_point.0, curr_point.1 + val),
            '1' => (curr_point.0, curr_point.1 - val),
            _ => panic!("Invalid instruction"),
        };

        points.push(curr_point);
    }

    points

}

/// Get the area of a polygon using the shoelace formula
fn shoelace(points: &[(i64, i64)]) -> f64 {
    let mut area = 0;

    for i in 0..(points.len() - 1) {
        area += points[i].0 * points[i + 1].1;
        area -= points[i].1 * points[i + 1].0;
    }
    area += points[points.len() - 1].0 * points[0].1;
    area -= points[points.len() - 1].1 * points[0].0;

    area = area.abs();

    area as f64 / 2.0
}

/// Get the number of boundary points of a polygon (points that are on the edge of the polygon)
fn boundary_points(points: &[(i64, i64)]) -> i64 {
    let mut points_count = points.len() as i64;

    for i in 0..(points.len() - 1) {
        // one of the coordinates is the same, so we can just add the difference of both coordinates
        // the result will be the distance between the two points
        // since we already counted all edge points, we just need to subtract 1
        // to get the number of points between the two points
        points_count += (points[i].0.abs_diff(points[i + 1].0) + points[i].1.abs_diff(points[i + 1].1) - 1) as i64;
    }
    points_count += (points[0].0.abs_diff(points[points.len() - 1].0) + points[0].1.abs_diff(points[points.len() - 1].1) - 1) as i64;

    points_count
}

/// Calculate the number of interior points of a polygon, using shoelace algorithm and pick's theorem
fn interior_points(points: &[(i64, i64)], bound_points: i64) -> i64 {
    let area = shoelace(points);

    // area = i + b/2 - 1
    // i = interior points
    // b = boundary points
    // i = area - b/2 + 1

    (area - (bound_points as f64) / 2.0 + 1.0) as i64
}
