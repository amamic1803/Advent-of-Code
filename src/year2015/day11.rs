use crate::{Day, Error};

day!(Day11, 11, "Corporate Policy");

impl Day11 {
    fn next_password(old_pass: &str) -> String {
        let mut word: Vec<u32> = old_pass.chars().map(Self::char_to_u32).collect();
        Self::fix_word(&mut word);
        Self::increment_word(&mut word);

        while !Self::two_pairs(&mut word) || !Self::increasing_seq(&mut word) {
            Self::increment_word(&mut word);
        }

        word.into_iter().map(Self::u32_to_char).collect()
    }

    const FORBIDDEN_CHARS: [char; 3] = ['i', 'o', 'l'];

    fn fix_word(word: &mut [u32]) {
        let limit = Self::char_to_u32('z') - Self::char_to_u32('a') + 1;

        for i in 0..word.len() {
            while Self::FORBIDDEN_CHARS.contains(&Self::u32_to_char(word[i])) {
                word[i] += 1;
                if word[word.len() - 1 - i] >= limit {
                    word[word.len() - 1 - i] = 0;
                }
            }
        }
    }

    fn increment_word(word: &mut [u32]) {
        let limit = Self::char_to_u32('z') - Self::char_to_u32('a') + 1;
        let mut i = 0;
        while i < 8 {
            let mut next = false;
            word[word.len() - 1 - i] += 1;
            if word[word.len() - 1 - i] >= limit {
                word[word.len() - 1 - i] = 0;
                next = true;
            }
            while Self::FORBIDDEN_CHARS.contains(&Self::u32_to_char(word[word.len() - 1 - i])) {
                word[word.len() - 1 - i] += 1;
                if word[word.len() - 1 - i] >= limit {
                    word[word.len() - 1 - i] = 0;
                    next = true;
                }
            }
            if !next {
                break;
            } else {
                i += 1;
            }
        }
    }

    fn two_pairs(word: &mut [u32]) -> bool {
        let mut last_pair_char: u32 = u32::MAX;
        let mut found = 0;
        let mut skip = 0;

        for i in 0..(word.len() - 1) {
            if skip > 0 {
                skip -= 1;
            } else if (word[i] == word[i + 1]) && (word[i] != last_pair_char) {
                found += 1;
                last_pair_char = word[i];
                skip = 1;
            }
        }

        found >= 2
    }

    fn increasing_seq(word: &mut [u32]) -> bool {
        for i in 0..(word.len() - 2) {
            if (word[i] + 1 == word[i + 1]) && (word[i + 1] + 1 == word[i + 2]) {
                return true;
            }
        }
        false
    }

    #[inline]
    fn char_to_u32(c: char) -> u32 {
        c as u32 - 'a' as u32
    }

    #[inline]
    fn u32_to_char(n: u32) -> char {
        char::from_u32(n + 'a' as u32).unwrap()
    }
}
impl Day for Day11 {
    fn id(&self) -> usize {
        self.id
    }
    fn title(&self) -> &str {
        self.title
    }
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Self::next_password(input.trim()))
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Self::next_password(&Self::next_password(input.trim())))
    }
}
