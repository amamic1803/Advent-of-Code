use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_08;
pub mod day_10;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_18;
pub mod day_21;
pub mod day_23;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
use day_05::day_05;
use day_06::day_06;
use day_08::day_08;
use day_10::day_10;
use day_14::day_14;
use day_15::day_15;
use day_16::day_16;
use day_18::day_18;
use day_21::day_21;
use day_23::day_23;

pub fn year_2017() -> Year {
    Year::new(
        2017,
        vec![
            day_01(),
            day_02(),
            day_03(),
            day_04(),
            day_05(),
            day_06(),
            day_08(),
            day_10(),
            day_14(),
            day_15(),
            day_16(),
            day_18(),
            day_21(),
            day_23(),
        ],
    )
}
