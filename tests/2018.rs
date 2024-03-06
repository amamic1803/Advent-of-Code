mod runner;
use runner::run_test;


#[test]
fn year2018_day01_part1() {
    run_test(2018, 1, 1, "411");
}

#[test]
fn year2018_day01_part2() {
    run_test(2018, 1, 2, "56360");
}

#[test]
fn year2018_day02_part1() {
    run_test(2018, 2, 1, "5000");
}

#[test]
fn year2018_day02_part2() {
    run_test(2018, 2, 2, "ymdrchgpvwfloluktajxijsqb");
}

#[test]
fn year2018_day03_part1() {
    run_test(2018, 3, 1, "103482");
}

#[test]
fn year2018_day03_part2() {
    run_test(2018, 3, 2, "686");
}
