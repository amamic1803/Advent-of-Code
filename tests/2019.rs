mod runner;
use runner::run_test;

#[test]
fn year2019_day01_part1() {
    run_test(2019, 1, 1, "3328306");
}

#[test]
fn year2019_day01_part2() {
    run_test(2019, 1, 2, "4989588");
}

#[test]
fn year2019_day02_part1() {
    run_test(2019, 2, 1, "5866663");
}

#[test]
fn year2019_day02_part2() {
    run_test(2019, 2, 2, "4259");
}

#[test]
fn year2019_day03_part1() {
    run_test(2019, 3, 1, "217");
}

#[test]
fn year2019_day03_part2() {
    run_test(2019, 3, 2, "3454");
}

#[test]
fn year2019_day04_part1() {
    run_test(2019, 4, 1, "1625");
}

#[test]
fn year2019_day04_part2() {
    run_test(2019, 4, 2, "1111");
}
