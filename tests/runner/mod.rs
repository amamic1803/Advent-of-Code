use std::process::Command;
use advent_of_code::get_challenges;

pub fn run_test(year: usize, day: usize, part: usize, expected: &str) {
    //! A function that runs a test for a given year, day, part and expected output.
    //! It tests both the library and the binary.

    // test library
    let output_lib = get_challenges().run(year, day, part, "").unwrap();

    // test binary
    let output_bin = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(year.to_string())
        .arg(day.to_string())
        .arg(part.to_string())
        .output()
        .unwrap();
    let output_bin = String::from(String::from_utf8(output_bin.stdout).unwrap().trim());

    assert_eq!(output_lib, expected);
    assert_eq!(output_bin, expected);
}
