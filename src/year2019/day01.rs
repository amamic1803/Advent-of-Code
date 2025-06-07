use crate::{Day, Error};

pub struct Day01;
impl Day01 {
    pub fn new() -> Self {
        Self
    }
}
impl Day for Day01 {
    fn id(&self) -> usize {
        1
    }

    fn title(&self) -> &str {
        "The Tyranny of the Rocket Equation"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;

        for num in input.trim().lines() {
            sum += num.parse::<u64>().unwrap() / 3;
            sum = sum.saturating_sub(2);
        }

        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut sum = 0;
        let fuel_for_fuel = |mut fuel: u64| {
            let mut sum = 0;

            while fuel > 2 {
                fuel /= 3;
                fuel = fuel.saturating_sub(2);
                sum += fuel;
            }

            sum
        };

        for num in input.trim().lines() {
            let module_fuel = (num.parse::<u64>().unwrap() / 3).saturating_sub(2);
            sum += module_fuel + fuel_for_fuel(module_fuel);
        }

        Ok(sum.to_string())
    }
}
