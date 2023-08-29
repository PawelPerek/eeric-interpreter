mod decoder;

use std::collections::HashMap;
use eeric::prelude::*;

use decoder::{ Decoder, LineClassification };

pub struct Interpreter;

impl Interpreter {
    pub fn compile(program: String) -> Result<Vec<Instruction>, Vec<(usize, String)>> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        let mut instructions = Vec::new();
        let mut errors = Vec::new();
        let mut line_address = 0;

        let mut instruction_lines = Vec::new();
        
        for line in program.lines() {
            let class = Decoder::classify(line);

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, line_address);
                },
                LineClassification::Instruction(instruction) => {
                    instruction_lines.push(instruction);
                    line_address += 4;
                },
                LineClassification::Empty => {},
            }
        }  

        let mut line_address = 0;

        for instruction in instruction_lines {
            let maybe_instruction = Decoder::decode(&instruction, &labels, line_address);

            match maybe_instruction {
                Ok(instruction) => instructions.push(instruction),
                Err(msg) => errors.push((line_address, msg)) 
            };

            line_address += 4;
        }   
        
        if errors.is_empty() {
            Ok(instructions)
        } else {
            Err(errors)
        }
    }
}
