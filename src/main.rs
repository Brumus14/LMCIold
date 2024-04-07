use std::fs;
use std::io::stdin;

enum Instruction {
    Add(u8),
    Subtract(u8),
    Store(u8),
    Load(u8),
    BranchAlways(usize),
    BranchZero(usize),
    BranchPositive(usize),
    Input,
    Output,
    Halt,
    Data,
}

fn main() {
    let source_code = fs::read_to_string("./test.lmc").unwrap();

    let instructions = parse_source_code(source_code);
    execute_instructions(instructions);
}

fn parse_source_code(source_code: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in source_code.lines() {
        let line_segments: Vec<&str> = line.split_whitespace().collect();

        for i in 0..line_segments.len() {
            match line_segments[i] {
                "ADD" => instructions.push(Instruction::Add(
                    line_segments[i + 1].parse::<u8>().unwrap(),
                )),
                "SUB" => instructions.push(Instruction::Subtract(
                    line_segments[i + 1].parse::<u8>().unwrap(),
                )),
                "STA" => instructions.push(Instruction::Store(
                    line_segments[i + 1].parse::<u8>().unwrap(),
                )),
                "LDA" => instructions.push(Instruction::Load(
                    line_segments[i + 1].parse::<u8>().unwrap(),
                )),
                "BRA" => instructions.push(Instruction::BranchAlways(
                    line_segments[i + 1].parse::<usize>().unwrap(),
                )),
                "BRZ" => instructions.push(Instruction::BranchZero(
                    line_segments[i + 1].parse::<usize>().unwrap(),
                )),
                "BRP" => instructions.push(Instruction::BranchPositive(
                    line_segments[i + 1].parse::<usize>().unwrap(),
                )),
                "INP" => instructions.push(Instruction::Input),
                "OUT" => instructions.push(Instruction::Output),
                "HLT" => instructions.push(Instruction::Halt),
                "DAT" => instructions.push(Instruction::Data),
                _ => (),
            };
        }
    }

    return instructions;
}

fn execute_instructions(instructions: Vec<Instruction>) {
    let mut program_counter = 0;
    let mut memory: Vec<u16> = vec![0; 100];
    let mut accumulator: i16 = 0;
    let mut halt = false;

    while !halt {
        match instructions[program_counter] {
            Instruction::Add(location) => {
                accumulator += memory[location as usize] as i16;
                program_counter += 1;
            }
            Instruction::Subtract(location) => {
                accumulator -= memory[location as usize] as i16;
                program_counter += 1;
            }
            Instruction::Store(location) => {
                memory[location as usize] = accumulator as u16;
                program_counter += 1;
            }
            Instruction::Load(location) => {
                accumulator = memory[location as usize] as i16;
                program_counter += 1;
            }
            Instruction::BranchAlways(new_counter) => {
                program_counter = new_counter;
            }
            Instruction::BranchZero(new_counter) => {
                if accumulator == 0 {
                    program_counter = new_counter;
                } else {
                    program_counter += 1;
                }
            }
            Instruction::BranchPositive(new_counter) => {
                if accumulator >= 0 {
                    program_counter = new_counter;
                } else {
                    program_counter += 1;
                }
            }
            Instruction::Input => {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                accumulator = input.trim().parse::<i16>().unwrap();
                program_counter += 1;
            }
            Instruction::Output => println!("{}", accumulator as u16),
            Instruction::Halt => {
                println!("Halted");
                halt = true;
                program_counter += 1;
            }
            Instruction::Data => todo!(),
        }

        println!("{}", accumulator);
    }
}
