use crate::{Day, Error};

day!(Day01, 1, "Inverse Captcha");

impl Day for Day01 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let sequence = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let mut sum: usize = 0;

        for (i, num) in sequence.iter().enumerate() {
            if num == sequence.get(i + 1).unwrap_or(&sequence[0]) {
                sum += *num as usize;
            }
        }

        Ok(sum.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let sequence = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let halfway = sequence.len() >> 1;
        let mut sum: usize = 0;

        for (i, num) in sequence.iter().enumerate() {
            if *num == sequence[(i + halfway) % sequence.len()] {
                sum += *num as usize;
            }
        }

        Ok(sum.to_string())
    }
}
