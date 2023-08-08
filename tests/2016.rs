mod runner;
use runner::run_test;


#[test]
fn year2016_day01_part1() {
    run_test(2016, 1, 1, "242");
}

#[test]
fn year2016_day01_part2() {
    run_test(2016, 1, 2, "150");
}