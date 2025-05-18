//! A library crate for solving the Advent of Code challenges.

pub mod challenges;
pub mod shared;

use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};
use crate::Error::UnavailablePart;

/// An enum representing the errors that can occur when using these structures.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested year is unavailable.
    UnavailableYear,
    /// The requested day is unavailable.
    UnavailableDay,
    /// The requested part is unavailable.
    UnavailablePart,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableYear => write!(f, "The requested year is unavailable."),
            Self::UnavailableDay => write!(f, "The requested day is unavailable."),
            Self::UnavailablePart => write!(f, "The requested part is unavailable."),
        }
    }
}
impl StdError for Error {}

/// A structure representing the Advent of Code.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AdventOfCode {
    /// A vector of years, each containing a vector of days.
    years: Vec<Year>,
}
impl AdventOfCode {
    /// Creates a new `Challenges` structure.
    /// # Arguments
    /// * `years` - A vector of years, each containing a vector of days.
    /// # Returns
    /// A `Challenges` structure.
    pub fn new(mut years: Vec<Year>) -> Self {

        Challenges::new(vec![
            year_2015(),
            year_2016(),
            year_2017(),
            year_2018(),
            year_2019(),
            year_2020(),
            year_2021(),
            year_2022(),
            year_2023(),
            year_2024(),
        ])


        years.sort_by_key(|year| year.year_num);
        Self { years }
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

    /// Shows the text of a challenge.
    /// # Arguments
    /// * `year_num` - The year of the challenge.
    /// * `day_num` - The day of the challenge.
    /// # Returns
    /// A `Result` containing a string with the text of the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableYear` - The year is unavailable.
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    pub fn show_text(&self, year_num: usize, day_num: usize) -> Result<String, ChallengeError> {
        match self.years.iter().position(|year| year.year_num == year_num) {
            Some(index) => Ok(self.years[index].show_text(day_num)?),
            None => Err(ChallengeError::UnavailableYear),
        }
    }

    /// Shows the input of a challenge.
    /// If input is empty, shows the default input.
    /// # Arguments
    /// * `year_num` - The year of the challenge.
    /// * `day_num` - The day of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the input to the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableYear` - The year is unavailable.
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    pub fn show_input(&self, year_num: usize, day_num: usize, input: &str) -> Result<String, ChallengeError> {
        match self.years.iter().position(|year| year.year_num == year_num) {
            Some(index) => Ok(self.years[index].show_input(day_num, input)?),
            None => Err(ChallengeError::UnavailableYear),
        }
    }

    /// Runs a challenge.
    /// # Arguments
    /// * `year_num` - The year of the challenge.
    /// * `day_num` - The day of the challenge.
    /// * `part_num` - The part of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableYear` - The year is unavailable.
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    /// * `ChallengeError::UnavailablePart` - The part is unavailable.
    pub fn run(&self, year_num: usize, day_num: usize, part_num: usize, input: &str) -> Result<String, ChallengeError> {
        self.benchmark(year_num, day_num, part_num, input).map(|(result, _)| result)
    }

    /// Benchmarks a challenge.
    /// # Arguments
    /// * `year_num` - The year of the challenge.
    /// * `day_num` - The day of the challenge.
    /// * `part_num` - The part of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a tuple with the answer to the challenge and the elapsed time or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableYear` - The year is unavailable.
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    /// * `ChallengeError::UnavailablePart` - The part is unavailable.
    pub fn benchmark(&self, year_num: usize, day_num: usize, part_num: usize, input: &str) -> Result<(String, Duration), ChallengeError> {
        match self.years.iter().position(|year| year.year_num == year_num) {
            Some(index) => Ok(self.years[index].benchmark(day_num, part_num, input)?),
            None => Err(ChallengeError::UnavailableYear),
        }
    }
}

/// A structure representing a year.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Year2 {
    /// The year number.
    year_num: usize,
    /// A vector of days.
    days: Vec<Day>,
}
impl Year2 {
    /// Creates a new `Year` structure.
    /// # Arguments
    /// * `year_num` - The year number.
    /// * `days` - A vector of days.
    /// # Returns
    /// A `Year` structure.
    pub fn new(year_num: usize, mut days: Vec<Day>) -> Self {
        days.sort_by_key(|day| day.day_num);
        Self { year_num, days }
    }

    /// Lists all available days in the year.
    /// # Returns
    /// A string containing the list of all available days in the year.
    pub fn list(&self) -> String {
        let mut list_str = String::new();

        list_str.push_str(&format!("Year {}\n", self.year_num));
        for day in &self.days {
            list_str.push_str(&format!("    {}\n", day.list()));
        }

        list_str
    }

    /// Shows the text of a challenge.
    /// # Arguments
    /// * `day_num` - The day of the challenge.
    /// # Returns
    /// A `Result` containing a string with the text of the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    pub fn show_text(&self, day_num: usize) -> Result<String, ChallengeError> {
        match self.days.iter().position(|day| day.day_num == day_num) {
            Some(index) => Ok(self.days[index].show_text()),
            None => Err(ChallengeError::UnavailableDay),
        }
    }

    /// Shows the input of a challenge.
    /// If input is empty, shows the default input.
    /// # Arguments
    /// * `day_num` - The day of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the input to the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    pub fn show_input(&self, day_num: usize, input: &str) -> Result<String, ChallengeError> {
        match self.days.iter().position(|day| day.day_num == day_num) {
            Some(index) => Ok(self.days[index].show_input(input)),
            None => Err(ChallengeError::UnavailableDay),
        }
    }
}

/// A trait representing a year.
trait Year {

    /// The year number.
    /// # Returns
    /// An integer representing the year.
    fn id(&self) -> usize;
    
    
    fn days(&self) -> impl Iterator<Item=&dyn Day>;
    fn day(&self, day: usize) -> Result<&dyn Day, Error> {
        self.days().find(|iter_day| iter_day.id() == day).ok_or(Error::UnavailableDay)
    }
    fn title(&self, day: usize) -> Result<&str, Error> {
        Ok(self.day(day)?.title())
    }
    fn text(&self, day: usize) -> Result<&str, Error> {
        Ok(self.day(day)?.text())
    }
    fn input<'a>(&'a self, day: usize, input: &'a str) -> Result<&'a str, Error> {
        Ok(self.day(day)?.input(input))
    }
    fn run(&self, day: usize, part: usize, input: &str) -> Result<String, Error> {
        self.day(day)?.run(part, input)
    }
    fn benchmark(&self, day: usize, part: usize, input: &str) -> Result<(String, Duration), Error> {
        self.day(day)?.benchmark(part, input)
    }
}

/// A trait representing a day.
trait Day {

    /// The number of the day in the advent calendar.
    /// # Returns
    /// An integer between 1 and 25.
    fn id(&self) -> usize;

    /// The title of the day's challenge.
    /// # Returns
    /// A string containing the title or an empty string if the title is unknown.
    fn title(&self) -> &str {
        self.text().trim().lines().next().unwrap_or("")
    }

    /// The text of the day's challenge.
    /// # Returns
    /// A string containing the full text of the challenge.
    fn text(&self) -> &str;

    /// The input of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A string containing the provided input if it is not empty, otherwise the default input.
    fn input<'a>(&'a self, input: &'a str) -> &'a str {
        if input.is_empty() {
            self.default_input()
        } else {
            input
        }
    }

    /// The default input of the day's challenge.
    /// # Returns
    /// A string containing the default input.
    fn default_input(&self) -> &str;

    /// Runs the specified part of the day's challenge.
    /// # Arguments
    /// * `part` - The part of the challenge to run (1 or 2).
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `Error`.
    /// # Errors
    /// * `Error::UnavailablePart` - The part is unavailable.
    fn run(&self, part: usize, input: &str) -> Result<String, Error> {
        let input = self.input(input);
        match part {
            1 => match self.part1(input) {
                Some(result) => Ok(result),
                None => Err(UnavailablePart),
            },
            2 => match self.part2(input) {
                Some(result) => Ok(result),
                None => Err(UnavailablePart),
            },
            _ => Err(UnavailablePart)
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
    /// An `Option<String>` containing the result of the challenge or `None`
    /// if the part is unavailable.
    fn part1(&self, input: &str) -> Option<String>;

    /// The second part of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// An `Option<String>` containing the result of the challenge or `None`
    /// if the part is unavailable.
    fn part2(&self, input: &str) -> Option<String>;
}
