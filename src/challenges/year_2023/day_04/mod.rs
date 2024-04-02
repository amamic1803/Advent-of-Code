use crate::shared::structures::Day;

pub fn day_04() -> Day {
    Day::new(
        4,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let cards = parse_input(input);
    let mut point_sum = 0;
    for card in cards {
        point_sum += card.points();
    }
    point_sum.to_string()
}

fn part2(input: &str) -> String {
    let mut cards = parse_input(input);

    for i in 0..cards.len() {
        let next_card_count = cards[i].winning_num_count();
        let new_copies = cards[i].copies;

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..=(i + next_card_count as usize) {
            cards[j].copies += new_copies;
        }
    }

    cards.iter().map(|c| c.copies).sum::<u32>().to_string()
}

struct Scratchcard {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    copies: u32,
}
impl Scratchcard {
    fn new(winning_numbers: Vec<u32>, my_numbers: Vec<u32>) -> Self {
        Scratchcard {
            winning_numbers,
            my_numbers,
            copies: 1,
        }
    }

    fn winning_num_count(&self) -> u32 {
        let mut same_numbers = 0;
        for my_number in &self.my_numbers {
            if self.winning_numbers.contains(my_number) {
                same_numbers += 1;
            }
        }
        same_numbers
    }

    fn points(&self) -> u32 {
        // points = floor(2^(n-1)) where n is the number of same numbers
        // can be simplified to 2^n / 2 (because this is integer division)
        // which can be simplified to (1 << n) >> 1
        (1 << self.winning_num_count()) >> 1
    }
}

fn parse_input(input: &str) -> Vec<Scratchcard> {
    let mut cards = Vec::new();

    for line in input.trim().lines() {
        let (_, cards_part) = line.split_once(':').unwrap();

        let (winning_cards_part, my_cards_part) = cards_part.split_once('|').unwrap();
        let winning_numbers = winning_cards_part
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let my_numbers = my_cards_part
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        cards.push(Scratchcard::new(winning_numbers, my_numbers));
    }

    cards
}
