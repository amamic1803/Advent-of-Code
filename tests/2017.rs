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
