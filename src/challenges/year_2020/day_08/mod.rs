use crate::shared::structures::Day;

pub fn day_08() -> Day {
    Day::new(
        8,
        include_str!("text.txt"),
        include_str!("input.txt"),
        part1,
        part2,
    )
}


fn part1(input: &str) -> String {
    let mut console = Console::new(input);
    console.execute().to_string()
}

fn part2(input: &str) -> String {
    let mut console = Console::new(input);
    console.execute_fixed().to_string()
}

#[derive(Debug)]
enum InstructionType {
    Acc,
    Jmp,
    Nop,
}
impl InstructionType {
    fn new(ins_type: &str) -> Self {
        match ins_type {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => panic!("Invalid instruction encountered"),
        }
    }
}
#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value: i32,
    executions: u8,
}
impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut ins_info = instruction.split_whitespace();
        let instruction_type = InstructionType::new(ins_info.next().unwrap());
        let value = ins_info.next().unwrap().parse().unwrap();
        Self {
            instruction_type,
            value,
            executions: 0
        }
    }
    fn reset(&mut self) {
        self.executions = 0;
    }
}
struct Console {
    instructions: Vec<Instruction>,
    ins_ptr: usize,
    max_ins_ptr: usize,
    accumulator: i32,
}
impl Console {
    fn new(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::new).collect();
        Self {
            instructions,
            ins_ptr: 0,
            max_ins_ptr: 0,
            accumulator: 0,
        }
    }
    fn execute(&mut self) -> i32 {
        while self.ins_ptr < self.instructions.len() && self.instructions[self.ins_ptr].executions < 1 {
            self.instructions[self.ins_ptr].executions += 1;
            match self.instructions[self.ins_ptr].instruction_type {
                InstructionType::Acc => {
                    self.accumulator += self.instructions[self.ins_ptr].value;
                    self.ins_ptr += 1;
                },
                InstructionType::Jmp => self.ins_ptr = (self.ins_ptr as i32 + self.instructions[self.ins_ptr].value) as usize,
                InstructionType::Nop => self.ins_ptr += 1,
            }
            self.max_ins_ptr = self.max_ins_ptr.max(self.ins_ptr);
        }
        self.accumulator
    }
    fn execute_fixed(&mut self) -> i32 {
        self.execute();
        self.instructions[self.max_ins_ptr] = Instruction::new("nop 0");
        self.reset();
        println!("{:?}", self.instructions);
        self.execute()
    }
    fn reset(&mut self) {
        self.ins_ptr = 0;
        self.max_ins_ptr = 0;
        self.accumulator = 0;
        self.instructions.iter_mut().for_each(|f| f.reset());
    }
}