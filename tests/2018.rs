mod runner;
use runner::run_test;


#[test]
fn year2018_day01_part1() {
    run_test(2018, 1, 1, "411");
}

#[test]
fn year2018_day01_part2() {
    run_test(2018, 1, 2, "56360");
}