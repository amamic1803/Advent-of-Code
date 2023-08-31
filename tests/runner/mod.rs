use advent_of_code::get_challenges;

pub fn run_test(year: usize, day: usize, part: usize, expected: &str) {
    //! A function that runs a test for a given year, day, part and expected output.
    //! It used to test both the library and the binary, but now it only tests library.
    //! Code for testing the binary is commented out.
    //! Binary is no longer tested because on macos runners,
    //! binary execution sometimes fails and no output is provided to test on.

    // test library
    let output_lib = get_challenges().run(year, day, part, "").unwrap();

    // use std::process::Command;
    // // test binary
    // let output_bin = Command::new("cargo")
    //     .arg("run")
    //     .arg("--")
    //     .arg(year.to_string())
    //     .arg(day.to_string())
    //     .arg(part.to_string())
    //     .output()
    //     .unwrap();
    // let output_bin = String::from(String::from_utf8(output_bin.stdout).unwrap().trim());
    // assert_eq!(output_bin, expected);

    assert_eq!(output_lib.trim(), expected.trim());
}
