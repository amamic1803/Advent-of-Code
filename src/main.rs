use std::fs::read_to_string;
use std::path::PathBuf;

use clap::{command, value_parser, Arg, ArgAction};

use advent_of_code::get_challenges;

fn main() {
    let argv = command!()
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("The year of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(2015..)),
        )
        .arg(
            Arg::new("day")
                .value_name("DAY")
                .help("The day of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(1..=25)),
        )
        .arg(
            Arg::new("part")
                .value_name("PART")
                .help("The part of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(1..=2)),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List all Advent of Code challenges")
                .conflicts_with_all(["solve", "text", "show_input"])
                .required(false),
        )
        .arg(
            Arg::new("solve")
                .short('s')
                .long("solve")
                .action(ArgAction::SetTrue)
                .help("Solve the Advent of Code challenge (default)")
                .conflicts_with_all(["text", "show_input", "list"])
                .required(false)
                .default_value_ifs([
                    ("text", "true", Some("false")),
                    ("show_input", "true", Some("false")),
                    ("list", "true", Some("false")),
                ])
                .default_value("true"),
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .action(ArgAction::SetTrue)
                .help("Show the text of the Advent of Code challenge")
                .conflicts_with_all(["solve", "show_input", "list"])
                .required(false),
        )
        .arg(
            Arg::new("show_input")
                .short('p')
                .long("show_input")
                .action(ArgAction::SetTrue)
                .help("Show the input of the Advent of Code challenge")
                .conflicts_with_all(["text", "solve", "list"])
                .required(false),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("Set the custom input to the Advent of Code challenge")
                .value_parser(value_parser!(PathBuf))
                .conflicts_with_all(["text", "list"])
                .required(false),
        )
        .get_matches();

    let year_num = argv.get_one::<u32>("year");
    let day_num = argv.get_one::<u32>("day");
    let part_num = argv.get_one::<u32>("part");
    let list_flag: bool = argv.get_flag("list");
    let solve_flag: bool = argv.get_flag("solve");
    let text_flag: bool = argv.get_flag("text");
    let show_input_flag: bool = argv.get_flag("show_input");
    let input = argv.get_one::<PathBuf>("input");

    let challenges_list = get_challenges();

    if list_flag {
        println!("{}", challenges_list.list());
    } else {
        let year_num: usize = match year_num {
            Some(year_num) => *year_num as usize,
            None => unreachable!("Year number is not set, but list_flag is not set either"),
        };
        let day_num: usize = match day_num {
            Some(day_num) => *day_num as usize,
            None => unreachable!("Day number is not set, but list_flag is not set either"),
        };
        let part_num: usize = match part_num {
            Some(part_num) => *part_num as usize,
            None => unreachable!("Part number is not set, but list_flag is not set either"),
        };

        let input: String = match input {
            Some(input) => match read_to_string(input) {
                Ok(input_str) => input_str,
                Err(err) => {
                    eprintln!("Error reading input file: {}", err);
                    std::process::exit(1);
                }
            },
            None => String::from(""),
        };

        if solve_flag {
            match challenges_list.run(year_num, day_num, part_num, &input) {
                Ok(result) => println!("{}", result),
                Err(err) => {
                    eprintln!("Error running challenge: {}", err);
                    std::process::exit(1);
                }
            }
        } else if text_flag {
            match challenges_list.show_text(year_num, day_num) {
                Ok(text) => println!("{}", text),
                Err(err) => {
                    eprintln!("Error showing challenge text: {}", err);
                    std::process::exit(1);
                }
            }
        } else if show_input_flag {
            match challenges_list.show_input(year_num, day_num, &input) {
                Ok(input) => println!("{}", input),
                Err(err) => {
                    eprintln!("Error showing challenge input: {}", err);
                    std::process::exit(1);
                }
            }
        } else {
            unreachable!("All flags are false, but at least one should be true");
        }
    }
}
