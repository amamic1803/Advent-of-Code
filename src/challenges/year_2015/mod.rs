use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
use day_05::day_05;
use day_06::day_06;
use day_07::day_07;
use day_08::day_08;
use day_09::day_09;
use day_10::day_10;
use day_11::day_11;
use day_12::day_12;
use day_13::day_13;
use day_14::day_14;
use day_15::day_15;
use day_16::day_16;
use day_17::day_17;
use day_18::day_18;
use day_19::day_19;
use day_20::day_20;
use day_21::day_21;
use day_22::day_22;
use day_23::day_23;
use day_24::day_24;
use day_25::day_25;

pub fn year_2015() -> Year {
    Year::new(
        2015,
        vec![
            day_01(),
            day_02(),
            day_03(),
            day_04(),
            day_05(),
            day_06(),
            day_07(),
            day_08(),
            day_09(),
            day_10(),
            day_11(),
            day_12(),
            day_13(),
            day_14(),
            day_15(),
            day_16(),
            day_17(),
            day_18(),
            day_19(),
            day_20(),
            day_21(),
            day_22(),
            day_23(),
            day_24(),
            day_25(),
        ],
    )
}
