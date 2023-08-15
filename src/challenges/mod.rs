//! A module containing solutions to the Advent of Code challenges.

use crate::shared::structures::Challenges;

pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2018;
pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;

use year_2015::year_2015;
use year_2016::year_2016;
use year_2017::year_2017;
use year_2018::year_2018;
use year_2019::year_2019;
use year_2020::year_2020;
use year_2021::year_2021;
use year_2022::year_2022;


/// Returns a `Challenges` struct containing all the challenges.
/// Call methods on this struct to interact with the challenges.
pub fn get_challenges() -> Challenges {
    Challenges::new(vec![
        year_2015(),
        year_2016(),
        year_2017(),
        year_2018(),
        year_2019(),
        year_2020(),
        year_2021(),
        year_2022(),
    ])
}
