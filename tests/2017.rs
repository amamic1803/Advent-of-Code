mod runner;
use runner::run_test;


#[test]
fn year2017_day01_part1() {
    run_test(2017, 1, 1, "1144");
}

#[test]
fn year2017_day01_part2() {
    run_test(2017, 1, 2, "1194");
}