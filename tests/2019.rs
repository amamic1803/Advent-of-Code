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