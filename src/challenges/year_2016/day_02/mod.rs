use crate::shared::structures::Day;

pub fn day_02() -> Day {
    Day::new(
        2,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let mut code = String::new();
    let mut position: [usize; 2] = [1, 1];

    for line in input.trim().lines() {
        for c in line.chars() {
            match c {
                'L' => { position[1] = position[1].saturating_sub(1) },
                'R' => { if position[1] < 2 { position[1] += 1 } },
                'U' => { position[0] = position[0].saturating_sub(1) },
                'D' => { if position[0] < 2 { position[0] += 1 } },
                _ => panic!("Invalid input!"),
            }
        }
        code.push(KEYPAD[position[0]][position[1]])
    }

    code
}

fn part2(input: &str) -> String {
    let mut code = String::new();
    let mut position: [usize; 2] = [2, 0];

    for line in input.trim().lines() {
        for c in line.chars() {
            match c {
                'L' => {
                    let new_pos = position[1].saturating_sub(1);
                    if KEYPAD2[position[0]][new_pos] != ' ' {
                        position[1] = new_pos;
                    }
                },
                'R' => {
                    let new_pos = if position[1] < 4 { position[1] + 1 } else { position[1] };
                    if KEYPAD2[position[0]][new_pos] != ' ' {
                        position[1] = new_pos;
                    }
                },
                'U' => {
                    let new_pos = position[0].saturating_sub(1);
                    if KEYPAD2[new_pos][position[1]] != ' ' {
                        position[0] = new_pos;
                    }
                },
                'D' => {
                    let new_pos = if position[0] < 4 { position[0] + 1 } else { position[0] };
                    if KEYPAD2[new_pos][position[1]] != ' ' {
                        position[0] = new_pos;
                    }
                },
                _ => panic!("Invalid input!"),
            }
        }
        code.push(KEYPAD2[position[0]][position[1]])
    }

    code
}

const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

const KEYPAD2: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];