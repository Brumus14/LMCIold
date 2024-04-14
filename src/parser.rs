use crate::data::*;
use std::collections::HashMap;

pub fn parse_source_code(source_code: String) -> Program {
    let mut program = Program::new();
    let initial_memory = &mut program.initial_memory;
    let instructions = &mut program.instructions;
    let mut variables: HashMap<String, u8> = HashMap::new();
    let mut variable_count = 0;
    let mut labels: HashMap<String, usize> = HashMap::new();

    for l in 0..source_code.lines().count() {
        let line = source_code.lines().nth(l).unwrap();
        let line_segments: Vec<&str> = line.split_whitespace().collect();

        for i in 0..line_segments.len() {
            match line_segments[i] {
                "ADD" | "SUB" | "STA" | "LDA" | "BRA" | "BRZ" | "BRP" | "INP" | "OUT" | "HLT" => {
                    if i == 1 {
                        labels.insert(line_segments[i - 1].to_string(), l);
                    }
                }
                "DAT" => {
                    variables.insert(line_segments[i - 1].to_string(), variable_count);
                    initial_memory[variable_count as usize] = if line_segments.len() < 3 {
                        0
                    } else {
                        line_segments[i + 1].parse::<u16>().unwrap()
                    };
                    variable_count += 1;
                }
                _ => (),
            };
        }
    }

    for line in source_code.lines() {
        let line_segments: Vec<&str> = line.split_whitespace().collect();

        for i in 0..line_segments.len() {
            match line_segments[i] {
                "ADD" => {
                    instructions.push(Instruction::Add(match line_segments[i + 1].parse::<u8>() {
                        Ok(v) => v,
                        Err(_) => *variables.get(&line_segments[i + 1].to_string()).unwrap(),
                    }))
                }
                "SUB" => instructions.push(Instruction::Subtract(
                    match line_segments[i + 1].parse::<u8>() {
                        Ok(v) => v,
                        Err(_) => *variables.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "STA" => instructions.push(Instruction::Store(
                    match line_segments[i + 1].parse::<u8>() {
                        Ok(v) => v,
                        Err(_) => *variables.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "LDA" => instructions.push(Instruction::Load(
                    match line_segments[i + 1].parse::<u8>() {
                        Ok(v) => v,
                        Err(_) => *variables.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "BRA" => instructions.push(Instruction::BranchAlways(
                    match line_segments[i + 1].parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => *labels.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "BRZ" => instructions.push(Instruction::BranchZero(
                    match line_segments[i + 1].parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => *labels.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "BRP" => instructions.push(Instruction::BranchPositive(
                    match line_segments[i + 1].parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => *labels.get(&line_segments[i + 1].to_string()).unwrap(),
                    },
                )),
                "INP" => instructions.push(Instruction::Input),
                "OUT" => instructions.push(Instruction::Output),
                "HLT" => instructions.push(Instruction::Halt),
                _ => continue,
            };
        }
    }

    program
}
