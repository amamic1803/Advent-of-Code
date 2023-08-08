use crate::challenges::Year;

mod day_01;

use day_01::day_01;


pub(crate) fn year_2017() -> Year {
    Year::new(
        2017,
        [
            Some(day_01()),
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