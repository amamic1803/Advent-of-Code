use crate::{Day, Year};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day02::Day02;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day04::Day04;
#[doc(inline)]
pub use day05::Day05;
#[doc(inline)]
pub use day06::Day06;
#[doc(inline)]
pub use day07::Day07;
#[doc(inline)]
pub use day08::Day08;
#[doc(inline)]
pub use day09::Day09;
#[doc(inline)]
pub use day10::Day10;
#[doc(inline)]
pub use day11::Day11;
#[doc(inline)]
pub use day12::Day12;
#[doc(inline)]
pub use day13::Day13;
#[doc(inline)]
pub use day14::Day14;
#[doc(inline)]
pub use day15::Day15;
#[doc(inline)]
pub use day16::Day16;
#[doc(inline)]
pub use day17::Day17;
#[doc(inline)]
pub use day18::Day18;
#[doc(inline)]
pub use day19::Day19;
#[doc(inline)]
pub use day20::Day20;
#[doc(inline)]
pub use day21::Day21;
#[doc(inline)]
pub use day22::Day22;
#[doc(inline)]
pub use day23::Day23;
#[doc(inline)]
pub use day24::Day24;
#[doc(inline)]
pub use day25::Day25;

pub struct Year2015 {
    days: Vec<Box<dyn Day>>,
}
impl Year2015 {
    pub fn new() -> Self {
        let mut new_self = Self {
            days: vec![
                Box::new(Day01::new()),
                Box::new(Day02::new()),
                Box::new(Day03::new()),
                Box::new(Day04::new()),
                Box::new(Day05::new()),
                Box::new(Day06::new()),
                Box::new(Day07::new()),
                Box::new(Day08::new()),
                Box::new(Day09::new()),
                Box::new(Day10::new()),
                Box::new(Day11::new()),
                Box::new(Day12::new()),
                Box::new(Day13::new()),
                Box::new(Day14::new()),
                Box::new(Day15::new()),
                Box::new(Day16::new()),
                Box::new(Day17::new()),
                Box::new(Day18::new()),
                Box::new(Day19::new()),
                Box::new(Day20::new()),
                Box::new(Day21::new()),
                Box::new(Day22::new()),
                Box::new(Day23::new()),
                Box::new(Day24::new()),
                Box::new(Day25::new()),
            ],
        };
        new_self.days.sort_by_key(|day| day.id());
        new_self
    }
}
impl Year for Year2015 {
    fn id(&self) -> usize {
        2015
    }
    fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Day> + 'a> {
        Box::new(self.days.iter().map(|day| day.as_ref()))
    }
}
