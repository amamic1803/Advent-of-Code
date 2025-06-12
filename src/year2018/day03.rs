use crate::{Day, Error};

day!(Day03, 3, "No Matter How You Slice It");

impl Day03 {
    const FABRIC_SIZE: usize = 1000;

    fn fabric_with_claims(claims: impl Iterator<Item = [usize; 4]>) -> Vec<[u8; Self::FABRIC_SIZE]> {
        let mut fabric = vec![[0_u8; Self::FABRIC_SIZE]; Self::FABRIC_SIZE];

        for claim in claims {
            let x_start = claim[0];
            let y_start = claim[1];
            let x_end = x_start + claim[2];
            let y_end = y_start + claim[3];
            for row in fabric[y_start..y_end].iter_mut() {
                row[x_start..x_end].iter_mut().for_each(|cell| *cell = (*cell).saturating_add(1));
            }
        }

        fabric
    }

    fn parse_input(input: &str) -> impl Iterator<Item = [usize; 4]> {
        input.lines().map(|line| {
            let mut arr = [0_usize; 4];
            let mut i = 0;
            for elem in line.split(&[' ', ',', 'x', ':']) {
                if let Ok(num) = elem.parse() {
                    arr[i] = num;
                    i += 1;
                }
            }
            arr
        })
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
        let fabric = Self::fabric_with_claims(Self::parse_input(input));

        let mut count = 0;
        for row in fabric.into_iter() {
            for cell in row.into_iter() {
                if cell >= 2 {
                    count += 1;
                }
            }
        }

        Ok(count.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let fabric = Self::fabric_with_claims(Self::parse_input(input));

        for (i, claim) in Self::parse_input(input).enumerate() {
            let x_start = claim[0];
            let y_start = claim[1];
            let x_end = x_start + claim[2];
            let y_end = y_start + claim[3];

            if fabric[y_start..y_end].iter().all(|row| row[x_start..x_end].iter().all(|&cell| cell == 1)) {
                return Ok((i + 1).to_string());
            }
        }

        Err(Error::NoSolutionFound)
    }
}
