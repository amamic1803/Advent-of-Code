use crate::shared::structures::Day;

pub fn day_09() -> Day {
    Day::new(9, include_str!("text.txt"), include_str!("input.txt"), part1, part2)
}

fn part1(input: &str) -> String {
    compress1(input).enumerate().map(|(i, block)| i as u64 * block as u64).sum::<u64>().to_string()
}

fn part2(input: &str) -> String {
    compress2(input).enumerate().map(|(i, block)| i as u64 * block as u64).sum::<u64>().to_string()
}

fn compress1(disc: &str) -> impl Iterator<Item = u32> {
    let mut disc = disc.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();
    if disc.len() % 2 == 0 {
        disc.pop();
    }
    let mut i = 0;
    std::iter::from_fn(move || loop {
        if i >= disc.len() {
            return None;
        }

        if disc[i] != 0 {
            disc[i] -= 1;
            return Some(i as u32 >> 1);
        }

        let last_ind = disc.len() - 1;
        if disc[last_ind] != 0 {
            if disc[i + 1] != 0 {
                disc[i + 1] -= 1;
                disc[last_ind] -= 1;
                return Some(last_ind as u32 >> 1);
            } else {
                i += 2;
            }
        } else {
            disc.pop();
            disc.pop();
        }
    })
}

fn compress2(disc: &str) -> impl Iterator<Item = u32> {
    let mut disc = disc.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>();
    let mut files = Vec::with_capacity(10_000);
    let mut empty = Vec::with_capacity(10_000);
    for (i, &b) in disc.iter().enumerate() {
        if i % 2 == 0 {
            files.push(b);
        } else {
            empty.push(b);
        }
    }
    if disc.len() % 2 == 0 {
        disc.pop();
    }
    //let mut out_ord = Vec::new();
    let mut i = 0;

    todo!("Implement the rest of the function");
    1..10
}
