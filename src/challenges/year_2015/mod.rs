use crate::challenges::Year;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
use day_05::day_05;
use day_06::day_06;
use day_07::day_07;
use day_08::day_08;
use day_09::day_09;


pub(crate) fn year_2015() -> Year {
    Year::new(
        2015,
        [
            Some(day_01()),
            Some(day_02()),
            Some(day_03()),
            Some(day_04()),
            Some(day_05()),
            Some(day_06()),
            Some(day_07()),
            Some(day_08()),
            Some(day_09()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
    )
}