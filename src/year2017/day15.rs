use crate::{Day, Error};

day!(Day15, 15, "Dueling Generators");

impl Day15 {
    const GEN_A_FACT: u64 = 16807;
    const GEN_B_FACT: u64 = 48271;
    const MOD: u64 = 2147483647;
    const ITERATIONS_1: u32 = 40_000_000;
    const ITERATIONS_2: u32 = 5_000_000;
    const GEN_A_MULT: u64 = 4;
    const GEN_B_MULT: u64 = 8;

    fn parse_input(input: &str) -> (u64, u64) {
        let mut a = 0;
        let mut b = 0;

        for line in input.trim().lines() {
            if line.contains('A') {
                a = line.split_whitespace().last().unwrap().parse().unwrap();
            } else if line.contains('B') {
                b = line.split_whitespace().last().unwrap().parse().unwrap();
            } else {
                panic!("Invalid input");
            }
        }

        (a, b)
    }
}
impl Day for Day15 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let (mut gen_a, mut gen_b) = Self::parse_input(input);
        let mut matches = 0;

        for _ in 0..Self::ITERATIONS_1 {
            gen_a = (gen_a * Self::GEN_A_FACT) % Self::MOD;
            gen_b = (gen_b * Self::GEN_B_FACT) % Self::MOD;

            if gen_a as u16 == gen_b as u16 {
                matches += 1;
            }
        }

        Ok(matches.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let (mut gen_a, mut gen_b) = Self::parse_input(input);
        let mut matches = 0;

        for _ in 0..Self::ITERATIONS_2 {
            gen_a = (gen_a * Self::GEN_A_FACT) % Self::MOD;
            while gen_a % Self::GEN_A_MULT != 0 {
                gen_a = (gen_a * Self::GEN_A_FACT) % Self::MOD;
            }
            gen_b = (gen_b * Self::GEN_B_FACT) % Self::MOD;
            while gen_b % Self::GEN_B_MULT != 0 {
                gen_b = (gen_b * Self::GEN_B_FACT) % Self::MOD;
            }

            if gen_a as u16 == gen_b as u16 {
                matches += 1;
            }
        }

        Ok(matches.to_string())
    }
}
