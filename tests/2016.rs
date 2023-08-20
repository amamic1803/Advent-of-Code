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

#[test]
fn year2016_day02_part1() {
    run_test(2016, 2, 1, "61529");
}

#[test]
fn year2016_day02_part2() {
    run_test(2016, 2, 2, "C2C28");
}

#[test]
fn year2016_day03_part1() {
    run_test(2016, 3, 1, "862");
}

#[test]
fn year2016_day03_part2() {
    run_test(2016, 3, 2, "1577");
}
