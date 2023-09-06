mod decoder;

use eeric::prelude::*;
use std::collections::HashMap;

use decoder::{Decoder, LineClassification};

pub struct Interpreter;

pub struct CompilationResult {
    pub instructions: Vec<Instruction>,
    pub instructions_addresses: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn compile(program: String) -> Result<CompilationResult, HashMap<usize, String>> {
        let mut labels = HashMap::new();
        let mut instructions = Vec::new();
        let mut instructions_addresses = HashMap::new();
        let mut line_address = 0;
        let mut raw_line_address = 0;

        let mut instruction_lines = Vec::new();

        instructions_addresses.insert(0, 0);

        for line in program.lines() {
            let class = Decoder::classify(line);
            raw_line_address += 4;

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, line_address);
                }
                LineClassification::Instruction(instruction) => {
                    instruction_lines.push(instruction);
                    line_address += 4;

                    instructions_addresses.insert(line_address, raw_line_address);
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
                    errors.insert(current_line, msg);
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
