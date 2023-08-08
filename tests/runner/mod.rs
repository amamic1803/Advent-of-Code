use std::process::Command;


pub fn run_test(year: usize, day: usize, part: usize, expected: &str) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(year.to_string())
        .arg(day.to_string())
        .arg(part.to_string())
        .output()
        .unwrap();
    let stdout = String::from(String::from_utf8(output.stdout).unwrap().trim());

    assert_eq!(stdout, expected);
}
