use crate::shared::structures::Year;


pub mod day_01;


use day_01::day_01;



pub fn year_2023() -> Year {
    Year::new(
        2023,
        vec![
            day_01(),
        ],
    )
}
