use crate::{Day, Error};
use std::iter::zip;

day!(Day06, 6, "Wait For It");

impl Day06 {
    const ACCELERATION: u64 = 1;

    fn parse_input(input: &str) -> Vec<(u64, u64)> {
        let mut input = input.trim().lines();
        let time_line = input
            .next()
            .unwrap()
            .trim_start_matches("Time:")
            .split_whitespace();
        let distance_line = input
            .next()
            .unwrap()
            .trim_start_matches("Distance:")
            .split_whitespace();

        let mut input_cases = Vec::new();
        for (time, distance) in zip(time_line, distance_line) {
            input_cases.push((time.parse().unwrap(), distance.parse().unwrap()));
        }
        input_cases
    }

    fn possible_victories(time: u64, distance: u64) -> u64 {
        // t_max = time
        // s_max = distance
        // v = velocity
        // s = maximum distance reached
        // v(t) = ACCELERATION * t
        // s(t) = v(t) * (t_max - t)
        // s(t) = ACCELERATION * t * (t_max - t)
        // s(t) = ACCELERATION * t * t_max - ACCELERATION * t^2
        // f(t) = s(t) - s_max = ACCELERATION * t * t_max - ACCELERATION * t^2 - s_max
        // we want f(t) to be positive (because we want to know when the distance is greater than the maximum distance)
        // since this is quadratic function with a < 0 we can find the roots, and we know that the function is positive for values between the roots

        // the number we are looking for is then the number of integers between the roots
        // if the roots are itself integers, they shouldn't be counted
        // (because the distance is equal to the maximum distance)

        let mut root_high = (-((Self::ACCELERATION * time) as f64)
            - (((Self::ACCELERATION * time).pow(2) - 4 * Self::ACCELERATION * distance) as f64)
                .sqrt())
            / (-2.0 * Self::ACCELERATION as f64);
        let mut root_low = (-((Self::ACCELERATION * time) as f64)
            + (((Self::ACCELERATION * time).pow(2) - 4 * Self::ACCELERATION * distance) as f64)
                .sqrt())
            / (-2.0 * Self::ACCELERATION as f64);

        if (root_low - root_low.round()).abs() <= 10e-8 {
            root_low += 1.0;
        } else {
            root_low = root_low.ceil();
        }
        if (root_high - root_high.round()).abs() <= 10e-8 {
            root_high -= 1.0;
        } else {
            root_high = root_high.floor();
        }

        root_high as u64 - root_low as u64 + 1
    }
}
impl Day for Day06 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let races = Self::parse_input(input);
        Ok(races
            .into_iter()
            .map(|(time, distance)| Self::possible_victories(time, distance))
            .product::<u64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let races = Self::parse_input(input);
        let time = races
            .iter()
            .map(|(time, _)| time.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        let distance = races
            .iter()
            .map(|(_, distance)| distance.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        Ok(Self::possible_victories(time, distance).to_string())
    }
}
