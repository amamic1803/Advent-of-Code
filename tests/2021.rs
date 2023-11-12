mod runner;
use runner::run_test;


#[test]
fn year2021_day01_part1() {
    run_test(2021, 1, 1, "1791");
}

#[test]
fn year2021_day01_part2() {
    run_test(2021, 1, 2, "1822");
}

#[test]
fn year2021_day02_part1() {
    run_test(2021, 2, 1, "1459206");
}

#[test]
fn year2021_day02_part2() {
    run_test(2021, 2, 2, "1320534480");
}

#[test]
fn year2021_day03_part1() {
    run_test(2021, 3, 1, "2595824");
}

#[test]
fn year2021_day03_part2() {
    run_test(2021, 3, 2, "2135254");
}
