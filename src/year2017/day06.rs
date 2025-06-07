use crate::{Day, Error};
use std::collections::{HashMap, HashSet};

pub struct Day06;
impl Day06 {
    pub fn new() -> Self {
        Self
    }

    const NUM_BANKS: usize = 16;

    fn parse_input(input: &str) -> [u16; Self::NUM_BANKS] {
        input.split_whitespace().map(|num| num.parse::<u16>().unwrap()).collect::<Vec<_>>().try_into().unwrap()
    }
}
impl Day for Day06 {
    fn id(&self) -> usize {
        6
    }

    fn title(&self) -> &str {
        "Memory Reallocation"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut memory_banks = Self::parse_input(input);
        let mut seen_states = HashSet::new();
        let mut cycles = 0;

        while !seen_states.contains(&memory_banks) {
            seen_states.insert(memory_banks);

            let mut max_index = 0;
            let mut max_value = memory_banks[0];
            for (i, val) in memory_banks.iter().enumerate().skip(1) {
                if *val > max_value {
                    max_index = i;
                    max_value = *val;
                }
            }
            memory_banks[max_index] = 0;

            let mut blocks = max_value;
            let mut index = max_index + 1;
            if index >= Self::NUM_BANKS {
                index = 0;
            }

            while blocks != 0 {
                memory_banks[index] += 1;
                blocks -= 1;
                index += 1;
                if index >= Self::NUM_BANKS {
                    index = 0;
                }
            }

            cycles += 1;
        }

        Ok(cycles.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut memory_banks = Self::parse_input(input);
        let mut seen_states = HashMap::new();
        let mut cycles = 0;

        while !seen_states.contains_key(&memory_banks) {
            seen_states.insert(memory_banks, cycles);

            let mut max_index = 0;
            let mut max_value = memory_banks[0];
            for (i, val) in memory_banks.iter().enumerate().skip(1) {
                if *val > max_value {
                    max_index = i;
                    max_value = *val;
                }
            }
            memory_banks[max_index] = 0;

            let mut blocks = max_value;
            let mut index = max_index + 1;
            if index >= Self::NUM_BANKS {
                index = 0;
            }

            while blocks != 0 {
                memory_banks[index] += 1;
                blocks -= 1;
                index += 1;
                if index >= Self::NUM_BANKS {
                    index = 0;
                }
            }

            cycles += 1;
        }

        Ok((cycles - seen_states.get(&memory_banks).unwrap()).to_string())
    }
}
