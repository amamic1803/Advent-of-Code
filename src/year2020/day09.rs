use crate::{Day, Error};

day!(Day09, 9, "Encoding Error");

impl Day09 {
    fn find_invalid_number_pos(data: &[u64]) -> usize {
        'outer: for i in 25..data.len() {
            let prev_25 = &data[i - 25..i];
            for &addend1 in prev_25 {
                if let Some(addend2) = data[i].checked_sub(addend1) {
                    if addend1 != addend2 && prev_25.contains(&addend2) {
                        continue 'outer;
                    }
                }
            }
            return i;
        }
        panic!("No error found")
    }
}
impl Day for Day09 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn part1(&self, input: &str) -> Result<String, Error> {
        let data = input
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let i = Self::find_invalid_number_pos(&data);
        Ok(data[i].to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let data = input
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let i = Self::find_invalid_number_pos(&data);
        let mut ptr1 = 0;
        let mut ptr2 = 0;
        let mut sum = data[0];

        while sum != data[i] {
            if sum < data[i] {
                ptr2 += 1;
                sum += data[ptr2];
            } else {
                #[allow(clippy::collapsible_else_if)]
                if ptr1 == ptr2 {
                    ptr1 += 1;
                    ptr2 += 1;
                    sum = data[ptr1];
                } else {
                    sum -= data[ptr1];
                    ptr1 += 1;
                }
            }
        }

        let min = data[ptr1..=ptr2].iter().min().unwrap();
        let max = data[ptr1..=ptr2].iter().max().unwrap();

        Ok((min + max).to_string())
    }
}
