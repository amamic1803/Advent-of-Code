use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;

use day_01::day_01;
use day_02::day_02;

pub fn year_2024() -> Year {
    Year::new(2024, vec![day_01(), day_02()])
}
