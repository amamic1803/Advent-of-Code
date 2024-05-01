use crate::shared::structures::Day;

pub fn day_09() -> Day {
    Day::new(9, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    let data = input.lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let i = find_invalid_number_pos(&data);
    data[i].to_string()
}

fn part2(input: &str) -> String {
    let data = input.lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let i = find_invalid_number_pos(&data);
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

    (min + max).to_string()
}

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
