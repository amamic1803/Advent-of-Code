use crate::shared::structures::Day;
use std::collections::HashSet;

pub fn day_19() -> Day {
    Day::new (
        19,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let (substitutions, molecule_str) = parse_input(input);
    let molecule_str = String::from(molecule_str);

    let mut new_molecules = HashSet::new();
    for substitution in substitutions {
        let positions = sub_string_positions(&molecule_str, substitution.0);

        for pos in positions {
            let mut new_molecule = molecule_str.clone();
            new_molecule.replace_range(pos..(pos + substitution.0.len()), substitution.1);
            new_molecules.insert(new_molecule);
        }
    }

    new_molecules.len().to_string()
}

fn part2(input: &str) -> String {
    String::new()
}

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let mut substitutions = Vec::new();
    let mut full_string: &str = "";

    for line in input.trim().lines() {
        if !line.is_empty() {
            if line.contains("=>") {
                let mut split = line.split(" => ");
                substitutions.push((split.next().unwrap(), split.next().unwrap()));
            } else {
                full_string = line;
            }
        }
    }

    (substitutions, full_string)
}

fn sub_string_positions(string: &str, sub_string: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = string.get(start..).unwrap().find(sub_string) {
        start += pos;
        positions.push(start);
        start += 1;
    }

    positions
}