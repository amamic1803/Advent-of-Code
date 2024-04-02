use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_08;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
use day_05::day_05;
use day_06::day_06;
use day_08::day_08;

pub fn year_2020() -> Year {
    Year::new(
        2020,
        vec![
            day_01(),
            day_02(),
            day_03(),
            day_04(),
            day_05(),
            day_06(),
            day_08(),
        ],
    )
}
