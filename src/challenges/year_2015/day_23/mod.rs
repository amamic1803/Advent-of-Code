use crate::shared::structures::Day;
use std::collections::HashMap;

pub fn day_23() -> Day {
    Day::new (
        23,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let instructions = parse_input(input);
    let mut registers: [usize; 2] = [0, 0];

    simulate(&mut registers, &instructions);

    registers[1].to_string()
}

fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let mut registers: [usize; 2] = [1, 0];

    simulate(&mut registers, &instructions);

    registers[1].to_string()
}

enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut reg_map = HashMap::new();
    reg_map.insert("a", 0);
    reg_map.insert("b", 1);

    for line in input.trim().lines() {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        match line[0] {
            "hlf" => instructions.push(Instruction::Hlf(*reg_map.get(line[1]).unwrap())),
            "tpl" => instructions.push(Instruction::Tpl(*reg_map.get(line[1]).unwrap())),
            "inc" => instructions.push(Instruction::Inc(*reg_map.get(line[1]).unwrap())),
            "jmp" => instructions.push(Instruction::Jmp(line[1].parse::<isize>().unwrap())),
            "jie" => instructions.push(Instruction::Jie(*reg_map.get(line[1].trim_end_matches(',')).unwrap(), line[2].parse::<isize>().unwrap())),
            "jio" => instructions.push(Instruction::Jio(*reg_map.get(line[1].trim_end_matches(',')).unwrap(), line[2].parse::<isize>().unwrap())),
            _ => panic!("Invalid instruction"),
        }
    }

    instructions
}

fn simulate(registers: &mut [usize; 2], instructions: &[Instruction]) {
    let mut pc: isize = 0;

    while pc >= 0 && pc < instructions.len() as isize {
        match instructions[pc as usize] {
            Instruction::Hlf(r) => registers[r] /= 2,
            Instruction::Tpl(r) => registers[r] *= 3,
            Instruction::Inc(r) => registers[r] += 1,
            Instruction::Jmp(o) => pc += o - 1,
            Instruction::Jie(r, o) => if registers[r] % 2 == 0 { pc += o - 1 },
            Instruction::Jio(r, o) => if registers[r] == 1 { pc += o - 1 },
        }

        pc += 1;
    }
}