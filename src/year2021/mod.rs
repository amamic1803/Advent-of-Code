use crate::{Day, Year};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day09;
pub mod day10;

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
pub use day09::Day09;
#[doc(inline)]
pub use day10::Day10;

pub struct Year2021 {
    days: Vec<Box<dyn Day>>,
}
impl Year2021 {
    pub fn new() -> Self {
        let mut new_self = Self {
            days: vec![
                Box::new(Day01::new()),
                Box::new(Day02::new()),
                Box::new(Day03::new()),
                Box::new(Day04::new()),
                Box::new(Day05::new()),
                Box::new(Day06::new()),
                Box::new(Day09::new()),
                Box::new(Day10::new()),
            ],
        };
        new_self.days.sort_by_key(|day| day.id());
        new_self
    }
}
impl Year for Year2021 {
    fn id(&self) -> usize {
        2021
    }
    fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Day> + 'a> {
        Box::new(self.days.iter().map(|day| day.as_ref()))
    }
}
