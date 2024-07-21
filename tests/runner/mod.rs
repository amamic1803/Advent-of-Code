use advent_of_code::get_challenges;

pub fn run_test(year: usize, day: usize, part: usize, expected: &str) {
    //! A function that runs a test for a given year, day, part and expected output.

    // test library
    let output_lib = get_challenges().run(year, day, part, "").unwrap();

    assert_eq!(output_lib.trim(), expected.trim());
}
