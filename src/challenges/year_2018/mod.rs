use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_05;
pub mod day_08;
pub mod day_12;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_05::day_05;
use day_08::day_08;
use day_12::day_12;

pub fn year_2018() -> Year {
    Year::new(
        2018,
        vec![day_01(), day_02(), day_03(), day_05(), day_08(), day_12()],
    )
}
