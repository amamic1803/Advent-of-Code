mod runner;
use runner::run_test;

#[test]
fn year2024_day01_part1() {
    run_test(2024, 1, 1, "1879048");
}

#[test]
fn year2024_day01_part2() {
    run_test(2024, 1, 2, "21024792");
}
