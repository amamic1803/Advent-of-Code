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
