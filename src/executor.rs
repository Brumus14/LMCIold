use crate::data::*;
use std::io::stdin;

pub fn execute_program(program: Program) {
    let instructions = program.instructions;

    let mut program_counter = 0;
    let mut memory = program.initial_memory;
    let mut accumulator: i16 = 0;
    let mut halt = false;

    while !halt {
        match instructions[program_counter].clone() {
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
            Instruction::Output => {
                println!("{}", accumulator as u16);
                program_counter += 1;
            }
            Instruction::Halt => {
                halt = true;
            }
        }
    }
}
