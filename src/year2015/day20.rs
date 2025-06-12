use crate::{Day, Error};

day!(Day20, 20, "Infinite Elves and Infinite Houses");

impl Day for Day20 {
    fn id(&self) -> usize {
        self.id
    }
    fn title(&self) -> &str {
        self.title
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        let input = input.trim().parse::<usize>().unwrap();
        let mut sieve = vec![0; input / 10];

        for i in 0..sieve.len() {
            let add_amount = i * 10;
            for j in (i..sieve.len()).step_by(i + 1) {
                sieve[j] += add_amount;
            }
            if sieve[i] >= input {
                return Ok((i + 1).to_string());
            }
        }

        Err(Error::NoSolutionFound)
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let input = input.trim().parse::<usize>().unwrap();
        let mut sieve = vec![0; (input as f64 / 11.0).ceil() as usize];

        for i in 0..sieve.len() {
            let add_amount = i * 11;
            for (loop_counter, j) in (i..sieve.len()).step_by(i + 1).enumerate() {
                if loop_counter >= 50 {
                    break;
                }
                sieve[j] += add_amount;
            }
            if sieve[i] >= input {
                return Ok((i + 1).to_string());
            }
        }

        Err(Error::NoSolutionFound)
    }
}
