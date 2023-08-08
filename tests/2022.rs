mod runner;
use runner::run_test;


#[test]
fn year2022_day01_part1() {
    run_test(2022, 1, 1, "69289");
}

#[test]
fn year2022_day01_part2() {
    run_test(2022, 1, 2, "205615");
}

#[test]
fn year2022_day02_part1() {
    run_test(2022, 2, 1, "11873");
}

#[test]
fn year2022_day02_part2() {
    run_test(2022, 2, 2, "12014");
}

#[test]
fn year2022_day03_part1() {
    run_test(2022, 3, 1, "7878");
}

#[test]
fn year2022_day03_part2() {
    run_test(2022, 3, 2, "2760");
}

#[test]
fn year2022_day04_part1() {
    run_test(2022, 4, 1, "540");
}

#[test]
fn year2022_day04_part2() {
    run_test(2022, 4, 2, "872");
}

#[test]
fn year2022_day05_part1() {
    run_test(2022, 5, 1, "NTWZZWHFV");
}

#[test]
fn year2022_day05_part2() {
    run_test(2022, 5, 2, "BRZGFVBTJ");
}

#[test]
fn year2022_day06_part1() {
    run_test(2022, 6, 1, "1287");
}

#[test]
fn year2022_day06_part2() {
    run_test(2022, 6, 2, "3716");
}

#[test]
fn year2022_day07_part1() {
    run_test(2022, 7, 1, "1432936");
}

#[test]
fn year2022_day07_part2() {
    run_test(2022, 7, 2, "272298");
}

#[test]
fn year2022_day08_part1() {
    run_test(2022, 8, 1, "1829");
}

#[test]
fn year2022_day08_part2() {
    run_test(2022, 8, 2, "291840");
}

#[test]
fn year2022_day09_part1() {
    run_test(2022, 9, 1, "5858");
}

#[test]
fn year2022_day09_part2() {
    run_test(2022, 9, 2, "2602");
}

#[test]
fn year2022_day10_part1() {
    run_test(2022, 10, 1, "12880");
}

#[test]
fn year2022_day10_part2() {
    let expected = "####..##....##..##..###....##.###..####.\n#....#..#....#.#..#.#..#....#.#..#.#....\n###..#.......#.#..#.#..#....#.#..#.###..\n#....#.......#.####.###.....#.###..#....\n#....#..#.#..#.#..#.#....#..#.#.#..#....\n#.....##...##..#..#.#.....##..#..#.####.";
    run_test(2022, 10, 2, expected);
}

#[test]
fn year2022_day11_part1() {
    run_test(2022, 11, 1, "55944");
}

#[test]
fn year2022_day11_part2() {
    run_test(2022, 11, 2, "15117269860");
}

#[test]
fn year2022_day12_part1() {
    run_test(2022, 12, 1, "528");
}

#[test]
fn year2022_day12_part2() {
    run_test(2022, 12, 2, "522");
}