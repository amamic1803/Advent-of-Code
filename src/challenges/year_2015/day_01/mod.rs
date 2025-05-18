use crate::Day;

struct Day01<'a> {
    text: &'a str,
    default_input: &'a str,
}
impl<'a> Day01<'a> {
    fn new() -> Self {
        Self {
            text: include_str!("text.txt"),
            default_input: include_str!("input.txt"),
        }
    }
}
impl<'a> Default for Day01<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Day for Day01<'a> {
    fn id(&self) -> usize { 1 }
    fn text(&self) -> &str { self.text }
    fn default_input(&self) -> &str { self.default_input }
    fn part1(&self, input: &str) -> Option<String> {
        let mut floor: isize = 0;
        for c in input.trim().chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }
        Some(floor.to_string())
    }
    fn part2(&self, input: &str) -> Option<String> {
        let mut floor: isize = 0;
        for (i, c) in input.trim().chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                return Some((i + 1).to_string());
            }
        }
        Some("Not found!".to_string())
    }
}
