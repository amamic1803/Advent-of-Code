mod runner;
use runner::run_test;


#[test]
fn year2023_day01_part1() {
    run_test(2023, 1, 1, "54630");
}

#[test]
fn year2023_day01_part2() {
    run_test(2023, 1, 2, "54770");
}
