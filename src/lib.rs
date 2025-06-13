//! A library crate for solving the Advent of Code challenges.

use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

macro_rules! day {
    ($struct_name:ident, $id:literal, $title:literal) => {
        #[doc = concat!("*", $title, "*")]
        #[derive(Copy, Clone)]
        pub struct $struct_name {
            id: usize,
            title: &'static str,
        }
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    id: $id,
                    title: $title,
                }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! year {
    ($struct_name:ident, $id:literal, $($day:ident),* ) => {
        #[doc = concat!("***", "Advent of Code ", $id, "***")]
        pub struct $struct_name {
            id: usize,
            days: Vec<Box<dyn crate::Day>>,
        }
        impl $struct_name {
            pub fn new() -> Self {
                let mut new_self = Self {
                    id: $id,
                    days: vec![
                        $(Box::new($day::new())),*
                    ],
                };
                new_self.days.sort_by_key(|day| day.id());
                new_self
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl crate::Year for $struct_name {
            fn id(&self) -> usize {
                self.id
            }
            fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn crate::Day> + 'a> {
                Box::new(self.days.iter().map(|day| day.as_ref()))
            }
        }
    };
}

pub mod shared;
pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;
pub mod year2024;

use year2015::Year2015;
use year2016::Year2016;
use year2017::Year2017;
use year2018::Year2018;
use year2019::Year2019;
use year2020::Year2020;
use year2021::Year2021;
use year2022::Year2022;
use year2023::Year2023;
use year2024::Year2024;

/// An enum representing the errors that can occur when using these structures.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested year is unavailable.
    UnavailableYear,
    /// The requested day is unavailable.
    UnavailableDay,
    /// The requested part is unavailable.
    UnavailablePart,
    /// There is no solution found for the challenge with the given input.
    NoSolutionFound,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableYear => write!(f, "The requested year is unavailable."),
            Self::UnavailableDay => write!(f, "The requested day is unavailable."),
            Self::UnavailablePart => write!(f, "The requested part is unavailable."),
            Self::NoSolutionFound => write!(
                f,
                "There is no solution found for the challenge with the given input."
            ),
        }
    }
}
impl StdError for Error {}

/// A structure representing the Advent of Code.
pub struct AdventOfCode {
    /// A vector of years, each containing a vector of days.
    years: Vec<Box<dyn Year>>,
}
impl AdventOfCode {
    /// Creates a new `Challenges` structure.
    /// # Arguments
    /// * `years` - A vector of years, each containing a vector of days.
    /// # Returns
    /// A `Challenges` structure.
    pub fn new() -> Self {
        let mut new_obj = Self {
            years: vec![
                Box::new(Year2015::new()),
                Box::new(Year2016::new()),
                Box::new(Year2017::new()),
                Box::new(Year2018::new()),
                Box::new(Year2019::new()),
                Box::new(Year2020::new()),
                Box::new(Year2021::new()),
                Box::new(Year2022::new()),
                Box::new(Year2023::new()),
                Box::new(Year2024::new()),
            ],
        };
        new_obj.years.sort_by_key(|year| year.id());
        new_obj
    }
}
impl Default for AdventOfCode {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for AdventOfCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Advent of Code:")?;

        for year in self.years() {
            write!(f, "\n  Year {:04}:", year.id())?;
            for day in year.days() {
                write!(f, "\n    --- Day {}: {} ---", day.id(), day.title())?;
            }
        }

        Ok(())
    }
}
impl AoC for AdventOfCode {
    fn years<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Year> + 'a> {
        Box::new(self.years.iter().map(|year| year.as_ref()))
    }
}

/// A trait representing the Advent of Code.
pub trait AoC: Send + Sync {
    /// Returns all available years in the Advent of Code.
    /// # Returns
    /// An iterator over the years, sorted by their year number in ascending order.
    fn years<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Year> + 'a>;

    /// Returns the year with the specified number.
    /// # Arguments
    /// * `year` - The year number to retrieve.
    /// # Returns
    /// A `Result` containing a reference to the year if it exists or an `Error`.
    /// # Errors
    /// * `Error::UnavailableYear` - The year is unavailable.
    fn year(&self, year: usize) -> Result<&dyn Year, Error> {
        for iter_year in self.years() {
            match iter_year.id().cmp(&year) {
                Ordering::Less => {}
                Ordering::Equal => return Ok(iter_year),
                Ordering::Greater => break,
            }
        }
        Err(Error::UnavailableYear)
    }

    /// Runs the specified part of the specified day's challenge in the specified year.
    /// # Arguments
    /// * `year` - The year of the challenge (2015, 2016, etc.).
    /// * `day` - The day of the challenge (1 to 25).
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `Error`.
    /// # Errors
    /// * `Error::UnavailableYear` - The year is unavailable.
    /// * `Error::UnavailableDay` - The day is unavailable.
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn run(&self, year: usize, day: usize, part: usize, input: &str) -> Result<String, Error> {
        self.year(year)?.run(day, part, input)
    }

    /// Benchmarks the specified part of the specified day's challenge in the specified year.
    /// # Arguments
    /// * `year` - The year of the challenge (2015, 2016, etc.).
    /// * `day` - The day of the challenge (1 to 25).
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a tuple with the answer to the challenge
    /// and the elapsed time or the `Error`.
    /// # Errors
    /// * `Error::UnavailableYear` - The year is unavailable.
    /// * `Error::UnavailableDay` - The day is unavailable.
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn benchmark(
        &self,
        year: usize,
        day: usize,
        part: usize,
        input: &str,
    ) -> Result<(String, Duration), Error> {
        self.year(year)?.benchmark(day, part, input)
    }
}

/// A trait representing a year.
pub trait Year: Send + Sync {
    /// The year of the Advent of Code.
    /// # Returns
    /// An integer representing the year.
    fn id(&self) -> usize;

    /// Returns available days in the year.
    /// # Returns
    /// An iterator over the days in the year, sorted by their day number in ascending order.
    fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Day> + 'a>;

    /// Returns the day with the specified number.
    /// # Arguments
    /// * `day` - The day number to retrieve.
    /// # Returns
    /// A `Result` containing a reference to the day if it exists or an `Error`.
    /// # Errors
    /// * `Error::UnavailableDay` - The day is unavailable.
    fn day(&self, day: usize) -> Result<&dyn Day, Error> {
        for iter_day in self.days() {
            match iter_day.id().cmp(&day) {
                Ordering::Less => {}
                Ordering::Equal => return Ok(iter_day),
                Ordering::Greater => break,
            }
        }
        Err(Error::UnavailableDay)
    }

    /// Runs the specified part of the specified day's challenge.
    /// # Arguments
    /// * `day` - The day of the challenge (1 to 25).
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `Error`.
    /// # Errors
    /// * `Error::UnavailableDay` - The day is unavailable.
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn run(&self, day: usize, part: usize, input: &str) -> Result<String, Error> {
        self.day(day)?.run(part, input)
    }

    /// Benchmarks the specified part of the specified day's challenge.
    /// # Arguments
    /// * `day` - The day of the challenge (1 to 25).
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a tuple with the answer to the challenge
    /// and the elapsed time or the `Error`.
    /// # Errors
    /// * `Error::UnavailableDay` - The day is unavailable.
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn benchmark(&self, day: usize, part: usize, input: &str) -> Result<(String, Duration), Error> {
        self.day(day)?.benchmark(part, input)
    }
}

/// A trait representing a day.
pub trait Day: Send + Sync {
    /// The number of the day in the advent calendar.
    /// # Returns
    /// An integer between 1 and 25.
    fn id(&self) -> usize;

    /// The title of the day's challenge.
    /// # Returns
    /// A string containing the title.
    fn title(&self) -> &str;

    /// Runs the specified part of the day's challenge.
    /// # Arguments
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `Error`.
    /// # Errors
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn run(&self, part: usize, input: &str) -> Result<String, Error> {
        match part {
            1 => self.part1(input),
            2 => self.part2(input),
            _ => Err(Error::UnavailablePart),
        }
    }

    /// Benchmarks the specified part of the day's challenge.
    /// # Arguments
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a tuple with the answer to the challenge
    /// and the elapsed time or the `Error`.
    /// # Errors
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn benchmark(&self, part: usize, input: &str) -> Result<(String, Duration), Error> {
        let instant = Instant::now();
        let result = self.run(part, input)?;
        let elapsed = instant.elapsed();
        Ok((result, elapsed))
    }

    /// The first part of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result<String, Error>` containing the result of the challenge or an `Error`
    /// # Errors
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn part1(&self, input: &str) -> Result<String, Error> {
        let _ = input; // suppress unused variable warning
        Err(Error::UnavailablePart)
    }

    /// The second part of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result<String, Error>` containing the result of the challenge or an `Error`
    /// # Errors
    /// * `Error::UnavailablePart` - The part is unavailable.
    /// * `Error::NoSolution` - There is no solution for the challenge with the given input.
    fn part2(&self, input: &str) -> Result<String, Error> {
        let _ = input; // suppress unused variable warning
        Err(Error::UnavailablePart)
    }
}
