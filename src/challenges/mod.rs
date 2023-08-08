mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;

use year_2015::year_2015;
use year_2016::year_2016;
use year_2017::year_2017;
use year_2018::year_2018;
use year_2019::year_2019;
use year_2020::year_2020;
use year_2021::year_2021;
use year_2022::year_2022;


pub(crate) fn challenges() -> Challenges {
    Challenges::new([
        Some(year_2015()),
        Some(year_2016()),
        Some(year_2017()),
        Some(year_2018()),
        Some(year_2019()),
        Some(year_2020()),
        Some(year_2021()),
        Some(year_2022()),
    ])
}


pub struct Challenges {
    pub(crate) years: [Option<Year>; 8],
}
impl Challenges {
    pub(crate) fn new(years: [Option<Year>; 8]) -> Self {
        Self {
            years,
        }
    }
    pub(crate) fn list(&self) {
        for year in self.years.iter().flatten() {
            year.list();
        }
    }
    pub(crate) fn show_text(&self, year_num: usize, day_num: usize) {
        if let Some(year) = self.years[year_num - 2015].as_ref() {
            year.show_text(day_num);
        } else {
            println!("Year {} not solved yet", year_num);
        }
    }
    pub(crate) fn show_input(&self, year_num: usize, day_num: usize, input: &str) {
        if let Some(year) = self.years[year_num - 2015].as_ref() {
            year.show_input(day_num, input);
        } else {
            println!("Year {} not solved yet", year_num);
        }
    }
    pub(crate) fn run_day(&self, year_num: usize, day_num: usize, part: usize, input: &str) {
        if let Some(year) = self.years[year_num - 2015].as_ref() {
            year.run_day(day_num, part, input);
        } else {
            println!("Year {} not solved yet", year_num);
        }
    }
}

pub struct Year {
    pub(crate) year_num: usize,
    pub(crate) days: [Option<Day>; 25],
}
impl Year {
    pub(crate) fn new(year_num: usize, days: [Option<Day>; 25]) -> Self {
        Self {
            year_num,
            days,
        }
    }
    pub(crate) fn show_name(&self) {
        println!("Year {}", self.year_num);
    }
    pub(crate) fn list(&self) {
        self.show_name();
        for day in self.days.iter().flatten() {
            day.show_name();
        }
    }
    pub(crate) fn show_text(&self, day_num: usize) {
        if let Some(day) = self.days[day_num - 1].as_ref() {
            day.show_text();
        } else {
            println!("Day {} not solved yet", day_num);
        }
    }
    pub(crate) fn show_input(&self, day_num: usize, input: &str) {
        if let Some(day) = self.days[day_num - 1].as_ref() {
            day.show_input(input);
        } else {
            println!("Day {} not solved yet", day_num);
        }
    }
    pub(crate) fn run_day(&self, day_num: usize, part: usize, input: &str) {
        if let Some(day) = self.days[day_num - 1].as_ref() {
            if part == 1 {
                day.part1(input);
            } else if part == 2 {
                day.part2(input);
            } else {
                unreachable!("Part should be 1 or 2");
            };
        } else {
            println!("Day {} not solved yet", day_num);
        }
    }
}

pub struct Day {
    pub(crate) name: String,
    pub(crate) text: String,
    pub(crate) default_input: String,
    pub(crate) part1: fn(&str),
    pub(crate) part2: fn(&str),
}
impl Day {
    pub(crate) fn new(challenge_text: &str, default_input: &str, part1: fn(&str), part2: fn(&str)) -> Self {
        Self {
            name: challenge_text.lines().next().unwrap_or("").to_string(),
            text: String::from(challenge_text),
            default_input: String::from(default_input),
            part1,
            part2,
        }
    }
    pub(crate) fn show_name(&self) {
        println!("    {}", self.name);
    }
    pub(crate) fn show_text(&self) {
        println!("{}", self.text);
    }
    pub(crate) fn show_input(&self, input: &str) {
        let input = if input.is_empty() {
            &self.default_input
        } else {
            input
        };
        println!("{}", input);
    }
    pub(crate) fn part1(&self, input: &str) {
        let input = if input.is_empty() {
            &self.default_input
        } else {
            input
        };
        (self.part1)(input);
    }
    pub(crate) fn part2(&self, input: &str) {
        let input = if input.is_empty() {
            &self.default_input
        } else {
            input
        };
        (self.part2)(input);
    }
}
