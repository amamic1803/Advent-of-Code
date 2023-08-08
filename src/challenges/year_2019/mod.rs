use crate::challenges::Year;

mod day_01;

use day_01::day_01;


pub(crate) fn year_2019() -> Year {
    Year::new(
        2019,
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