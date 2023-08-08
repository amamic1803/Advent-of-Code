mod runner;
use runner::run_test;


#[test]
fn year2020_day01_part1() {
    run_test(2020, 1, 1, "299299");
}

#[test]
fn year2020_day01_part2() {
    run_test(2020, 1, 2, "287730716");
}