use crate::{Day, Error};

day!(Day17, 17, "Spinlock");

impl Day17 {
    const PART1_LIMIT: usize = 2017;
    const PART2_LIMIT: usize = 50_000_000;
}
impl Day for Day17 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let steps = input.trim().parse::<usize>().unwrap();
        let mut circular_buffer = Vec::with_capacity(Self::PART1_LIMIT + 1);
        circular_buffer.push(0);
        let mut i = 0;
        for n in 1..=Self::PART1_LIMIT {
            i = (i + steps) % circular_buffer.len() + 1;
            circular_buffer.insert(i, n);
        }

        Ok(circular_buffer[circular_buffer.iter().position(|&elem| elem == Self::PART1_LIMIT).unwrap() + 1].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let steps = input.trim().parse::<usize>().unwrap();
        let mut circular_buffer = 1;
        let mut result = 0;
        let mut i = 0;
        for n in 1..=Self::PART2_LIMIT {
            i = (i + steps) % circular_buffer + 1;
            if i == 1 {
                result = n;
            }
            circular_buffer += 1;
        }

        Ok(result.to_string())
    }
}
