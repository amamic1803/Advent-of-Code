use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_03;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;


pub fn year_2017() -> Year {
    Year::new(
        2017,
        vec![
            day_01(),
            day_02(),
            day_03(),
        ],
    )
}