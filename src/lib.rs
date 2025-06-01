//! A library crate for solving the Advent of Code challenges.

pub mod shared;
pub mod year2015;
pub mod year2016;

use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::time::{Duration, Instant};

use year2015::Year2015;
use year2016::Year2016;

/// An enum representing the errors that can occur when using these structures.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested year is unavailable.
    UnavailableYear,
    /// The requested day is unavailable.
    UnavailableDay,
    /// The requested part is unavailable.
    UnavailablePart,
    /// There is no solution for the challenge with the given input.
    NoSolution,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableYear => write!(f, "The requested year is unavailable."),
            Self::UnavailableDay => write!(f, "The requested day is unavailable."),
            Self::UnavailablePart => write!(f, "The requested part is unavailable."),
            Self::NoSolution => write!(f, "There is no solution for the challenge with the given input."),
        }
    }
}
impl StdError for Error {}

/// A structure representing the Advent of Code.
pub struct AdventOfCodeInstance {
    /// A vector of years, each containing a vector of days.
    years: Vec<Box<dyn Year>>,
}
impl AdventOfCodeInstance {
    /// Creates a new `Challenges` structure.
    /// # Arguments
    /// * `years` - A vector of years, each containing a vector of days.
    /// # Returns
    /// A `Challenges` structure.
    pub fn new() -> Self {
        let mut new_obj = Self {
            years: vec![Box::new(Year2015::new()), Box::new(Year2016::new())],
        };
        new_obj.years.sort_by_key(|year| year.id());
        new_obj
    }

    /// Lists all available (solved) challenges.
    /// # Returns
    /// A string containing the list of all available challenges.
    pub fn list(&self) -> String {
        let mut year_str = String::new();
        for year in &self.years {
            year_str.push_str(&year.list());
        }
        year_str.pop();

        let mut longest_line: usize = 0;
        for line in year_str.trim().lines() {
            let length = line.chars().count();
            if length > longest_line {
                longest_line = length;
            }
        }

        if longest_line < 22 {
            longest_line = 22;
        } else if longest_line % 2 == 1 {
            longest_line += 1;
        }

        let mut list_str = String::new();
        for _ in 0..longest_line {
            list_str.push('#');
        }
        list_str.push('\n');
        for _ in 0..((longest_line - 16) / 2) {
            list_str.push('#');
        }
        list_str.push_str(" Advent of Code ");
        for _ in 0..((longest_line - 16) / 2) {
            list_str.push('#');
        }
        list_str.push('\n');
        for _ in 0..longest_line {
            list_str.push('#');
        }
        list_str.push('\n');
        list_str.push_str(&year_str);

        list_str
    }
}
impl AdventOfCode for AdventOfCodeInstance {
    fn years<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Year> + 'a> {
        Box::new(self.years.iter().map(|year| year.as_ref()))
    }
}

/// A trait representing the Advent of Code.
pub trait AdventOfCode: Send + Sync {
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
    fn benchmark(&self, year: usize, day: usize, part: usize, input: &str) -> Result<(String, Duration), Error> {
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
        Err(Error::UnavailablePart)
    }
}
