use crate::{Day, Error};

pub struct Day16;
impl Day16 {
    pub fn new() -> Self {
        Self
    }

    const LEN1: usize = 272;
    const LEN2: usize = 35651584;

    /// Parse the input into a vector of bools
    /// true = 1
    /// false = 0
    fn parse_input(input: &str) -> Vec<bool> {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("Invalid input"),
            })
            .collect()
    }

    fn generate_data(data: &mut Vec<bool>, len: usize) {
        while data.len() < len {
            let data_len = data.len();

            data.reserve_exact(data_len + 1);
            data.push(false);

            for i in (0..data_len).rev() {
                data.push(!data[i]);
            }
        }

        data.truncate(len);
    }

    fn generate_checksum(data: &mut Vec<bool>) {
        while data.len() % 2 == 0 {
            for i in 0..(data.len() / 2) {
                let orig_loc = i * 2;
                data[i] = data[orig_loc] == data[orig_loc + 1];
            }
            data.truncate(data.len() / 2);
        }
    }

    /// Convert the bools into a string of 1s and 0s
    fn pretty_str(data: &[bool]) -> String {
        let mut s = String::with_capacity(data.len());
        for b in data {
            if *b {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s
    }
}
impl Day for Day16 {
    fn id(&self) -> usize {
        16
    }

    fn title(&self) -> &str {
        "Dragon Checksum"
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut data = Self::parse_input(input);
        Self::generate_data(&mut data, Self::LEN1);
        Self::generate_checksum(&mut data);

        Ok(Self::pretty_str(&data))
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut data = Self::parse_input(input);
        Self::generate_data(&mut data, Self::LEN2);
        Self::generate_checksum(&mut data);

        Ok(Self::pretty_str(&data))
    }
}
