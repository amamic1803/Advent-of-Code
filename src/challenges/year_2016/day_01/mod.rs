use crate::shared::structures::Day;

pub fn day_01() -> Day {
    Day::new(
        1,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}

fn part1(input: &str) -> String {
    let end_position = calculate_position(parse_input(input));
    format!("{}", end_position.0.abs() + end_position.1.abs())
}

fn part2(input: &str) -> String {
    let instructions: Vec<(u8, usize)> = parse_input(input);
    let mut visited_positions: Vec<(isize, isize)> = Vec::new();

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut direction: u8 = 0;
    // 0 = North
    // 1 = East
    // 2 = South
    // 3 = West

    for instruction in instructions {
        direction = change_direction(direction, instruction.0);
        for _ in 0..instruction.1 {
            match direction {
                0 => y += 1,
                1 => x += 1,
                2 => y -= 1,
                3 => x -= 1,
                _ => panic!("Invalid direction value"),
            }
            if visited_positions.contains(&(x, y)) {
                return format!("{}", x.abs() + y.abs());
            } else {
                visited_positions.push((x, y));
            }
        }
    }

    "No position visited twice!".to_string()
}

fn parse_input(input: &str) -> Vec<(u8, usize)> {
    let mut instructions: Vec<(u8, usize)> = Vec::new();

    let mut instruction: (u8, usize) = (0, 0);
    let mut number: String = String::new();
    for c in input.trim().chars() {
        match c {
            'R' => instruction.0 = 1,
            'L' => instruction.0 = 0,
            ',' => {
                instruction.1 = number.parse::<usize>().unwrap();
                number.clear();
                instructions.push(instruction);
                instruction = (0, 0);
            }
            _ => {
                if c.is_ascii_digit() {
                    number.push(c);
                }
            }
        }
    }
    if !number.is_empty() {
        instruction.1 = number.parse::<usize>().unwrap();
        instructions.push(instruction)
    }

    instructions
}

fn calculate_position(instructions: Vec<(u8, usize)>) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut direction: u8 = 0;
    // 0 = North
    // 1 = East
    // 2 = South
    // 3 = West

    for instruction in instructions {
        direction = change_direction(direction, instruction.0);
        match direction {
            0 => y += isize::try_from(instruction.1).unwrap(),
            1 => x += isize::try_from(instruction.1).unwrap(),
            2 => y -= isize::try_from(instruction.1).unwrap(),
            3 => x -= isize::try_from(instruction.1).unwrap(),
            _ => panic!("Invalid direction value"),
        }
    }

    (x, y)
}

fn change_direction(direction: u8, turn: u8) -> u8 {
    match turn {
        0 => {
            if direction == 0 {
                3
            } else {
                direction - 1
            }
        }
        1 => {
            if direction == 3 {
                0
            } else {
                direction + 1
            }
        }
        _ => panic!("Invalid turn value"),
    }
}
