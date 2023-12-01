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

#[test]
fn year2017_day02_part1() {
    run_test(2017, 2, 1, "53978");
}

#[test]
fn year2017_day02_part2() {
    run_test(2017, 2, 2, "314");
}

#[test]
fn year2017_day03_part1() {
    run_test(2017, 3, 1, "430");
}

#[test]
fn year2017_day03_part2() {
    run_test(2017, 3, 2, "312453");
}

#[test]
fn year2017_day04_part1() {
    run_test(2017, 4, 1, "383");
}

#[test]
fn year2017_day04_part2() {
    run_test(2017, 4, 2, "265");
}

#[test]
fn year2017_day05_part1() {
    run_test(2017, 5, 1, "387096");
}

#[test]
fn year2017_day05_part2() {
    run_test(2017, 5, 2, "28040648");
}
