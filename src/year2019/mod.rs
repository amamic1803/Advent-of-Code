use crate::{Day, Year};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day02::Day02;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day04::Day04;

pub struct Year2019 {
    days: Vec<Box<dyn Day>>,
}
impl Year2019 {
    pub fn new() -> Self {
        let mut new_self = Self {
            days: vec![Box::new(Day01::new()), Box::new(Day02::new()), Box::new(Day03::new()), Box::new(Day04::new())],
        };
        new_self.days.sort_by_key(|day| day.id());
        new_self
    }
}
impl Year for Year2019 {
    fn id(&self) -> usize {
        2019
    }
    fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Day> + 'a> {
        Box::new(self.days.iter().map(|day| day.as_ref()))
    }
}
