mod runner;
use runner::run_test;


#[test]
fn year2021_day01_part1() {
    run_test(2021, 1, 1, "1791");
}

#[test]
fn year2021_day01_part2() {
    run_test(2021, 1, 2, "1822");
}