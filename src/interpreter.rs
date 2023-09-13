mod decoder;

use eeric::prelude::*;
use std::collections::HashMap;

use decoder::{Decoder, LineClassification};

pub struct Interpreter;

pub struct CompilationResult {
    pub instructions: Vec<Instruction>,
    pub instructions_addresses: Vec<usize>
}

impl Interpreter {
    pub fn compile(program: String) -> Result<CompilationResult, HashMap<usize, String>> {
        let mut labels = HashMap::new();
        let mut instructions = Vec::new();
        let mut instructions_addresses = Vec::new();
        let mut program_line_address = 0;

        let mut instruction_lines = Vec::new();

        for (line_address, line) in program.lines().enumerate() {
            let class = Decoder::classify(line);

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, program_line_address);
                }
                LineClassification::Instruction(instruction) => {
                    program_line_address += 4;
                    instruction_lines.push(instruction);
                    instructions_addresses.push(line_address);
                }
                LineClassification::Empty => {}
            }
        }

        let mut errors = HashMap::new();

        for (current_line, instruction) in instruction_lines.into_iter().enumerate() {
            let maybe_instruction = Decoder::decode(&instruction, &labels, current_line * 4);

            match maybe_instruction {
                Ok(instruction) => instructions.push(instruction),
                Err(msg) => {
                    errors.insert(instructions_addresses[current_line], msg);
                }
            };
        }

        if errors.is_empty() {
            Ok(CompilationResult {
                instructions,
                instructions_addresses,
            })
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_map_calculation() {
        let input = r#"
        addi x1, x0, 123
        loop:
        inner_loop:
            add x1, x1, x1
            bnez x1, loop
        "#.trim_start();

        let compilation_result = Interpreter::compile(input.to_owned()).unwrap();
        
        assert_eq!(
            compilation_result.instructions, 
            vec![
                Instruction::Addi(format::I { rd: 1, rs1: 0, imm12: 123 }),
                Instruction::Add(format::R { rd: 1, rs1: 1, rs2: 1 }),
                Instruction::Bne(format::S { rs1: 1, rs2: 0, imm12: -8 }),   
            ]
        );

        assert_eq!(
            compilation_result.instructions_addresses, 
            vec![
                0, // addi x1, x0, 123
                3, // add x1, x1, x1
                4  // bnez x1, loop
            ]
        );
    }
}