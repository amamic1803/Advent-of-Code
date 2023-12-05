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

#[test]
fn year2023_day02_part1() {
    run_test(2023, 2, 1, "2265");
}

#[test]
fn year2023_day02_part2() {
    run_test(2023, 2, 2, "64097");
}

#[test]
fn year2023_day03_part1() {
    run_test(2023, 3, 1, "556367");
}

#[test]
fn year2023_day03_part2() {
    run_test(2023, 3, 2, "89471771");
}

#[test]
fn year2023_day04_part1() {
    run_test(2023, 4, 1, "25183");
}

#[test]
fn year2023_day04_part2() {
    run_test(2023, 4, 2, "5667240");
}

#[test]
fn year2023_day05_part1() {
    run_test(2023, 5, 1, "323142486");
}

#[test]
fn year2023_day05_part2() {
    run_test(2023, 5, 2, "79874951");
}
