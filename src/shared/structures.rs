//! A module containing structures used to represent challenges.

use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// An enum representing the errors that can occur when using these structures.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ChallengeError {
    /// The requested year is unavailable.
    UnavailableYear,
    /// The requested day is unavailable.
    UnavailableDay,
}
impl Display for ChallengeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableYear => write!(f, "The requested year is unavailable."),
            Self::UnavailableDay => write!(f, "The requested day is unavailable."),
        }
    }
}
impl Error for ChallengeError {}

/// A structure representing the challenges.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Challenges {
    /// A vector of years, each containing a vector of days.
    years: Vec<Year>,
}
impl Challenges {
    /// Creates a new `Challenges` structure.
    /// # Arguments
    /// * `years` - A vector of years, each containing a vector of days.
    /// # Returns
    /// A `Challenges` structure.
    pub fn new(mut years: Vec<Year>) -> Self {
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
    pub fn run(&self, year_num: usize, day_num: usize, part_num: usize, input: &str) -> Result<String, ChallengeError> {
        match self.years.iter().position(|year| year.year_num == year_num) {
            Some(index) => Ok(self.years[index].run(day_num, part_num, input)?),
            None => Err(ChallengeError::UnavailableYear),
        }
    }
}

/// A structure representing a year.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Year {
    /// The year number.
    year_num: usize,
    /// A vector of days.
    days: Vec<Day>,
}
impl Year {
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

    /// Runs a challenge.
    /// # Arguments
    /// * `day_num` - The day of the challenge.
    /// * `part_num` - The part of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A `Result` containing a string with the answer to the challenge or the `ChallengeError`.
    /// # Errors
    /// * `ChallengeError::UnavailableDay` - The day is unavailable.
    pub fn run(&self, day_num: usize, part_num: usize, input: &str) -> Result<String, ChallengeError> {
        match self.days.iter().position(|day| day.day_num == day_num) {
            Some(index) => Ok(self.days[index].run(part_num, input)),
            None => Err(ChallengeError::UnavailableDay),
        }
    }
}

/// A structure representing a day.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Day {
    /// The day number.
    day_num: usize,
    /// The name of the challenge.
    name: String,
    /// The text of the challenge.
    text: String,
    /// The default input to the challenge.
    default_input: String,
    /// The function to run part 1 of the challenge.
    part1: fn(&str) -> String,
    /// The function to run part 2 of the challenge.
    part2: fn(&str) -> String,
}
impl Day {
    /// Creates a new `Day` structure.
    /// # Arguments
    /// * `day_num` - The day number.
    /// * `text` - The text of the challenge.
    /// * `default_input` - The default input to the challenge.
    /// * `part1` - The function to run part 1 of the challenge.
    /// * `part2` - The function to run part 2 of the challenge.
    /// # Returns
    /// A `Day` structure.
    pub fn new(day_num: usize, text: &str, default_input: &str, part1: fn(&str) -> String, part2: fn(&str) -> String) -> Self {
        Self {
            day_num,
            name: text.trim().lines().next().unwrap_or("").to_string(),
            text: text.to_string(),
            default_input: default_input.to_string(),
            part1,
            part2,
        }
    }

    /// Lists the challenge.
    /// # Returns
    /// A string containing the name of the challenge.
    pub fn list(&self) -> String {
        self.name.clone()
    }

    /// Shows the text of a challenge.
    /// # Returns
    /// A string with the text of the challenge.
    pub fn show_text(&self) -> String {
        self.text.clone()
    }

    /// Shows the input of a challenge.
    /// If input is empty, shows the default input.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A string with the input to the challenge.
    pub fn show_input(&self, input: &str) -> String {
        if input.is_empty() {
            self.default_input.clone()
        } else {
            input.to_string()
        }
    }

    /// Runs a challenge.
    /// `part_num` should be 1 or 2. If it is not, the function will return an empty string.
    /// # Arguments
    /// * `part_num` - The part of the challenge.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// A string with the answer to the challenge.
    pub fn run(&self, part_num: usize, input: &str) -> String {
        let input = if input.is_empty() { &self.default_input } else { input };
        match part_num {
            1 => (self.part1)(input),
            2 => (self.part2)(input),
            _ => String::new(),
        }
    }
}
