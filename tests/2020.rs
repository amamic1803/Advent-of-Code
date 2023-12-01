mod runner;
use runner::run_test;


#[test]
fn year2020_day01_part1() {
    run_test(2020, 1, 1, "299299");
}

#[test]
fn year2020_day01_part2() {
    run_test(2020, 1, 2, "287730716");
}

#[test]
fn year2020_day02_part1() {
    run_test(2020, 2, 1, "580");
}

#[test]
fn year2020_day02_part2() {
    run_test(2020, 2, 2, "611");
}

#[test]
fn year2020_day03_part1() {
    run_test(2020, 3, 1, "228");
}

#[test]
fn year2020_day03_part2() {
    run_test(2020, 3, 2, "6818112000");
}
