mod decoder;

use std::collections::HashMap;
use eeric::prelude::*;

use decoder::{ Decoder, LineClassification };

pub struct Interpreter;

impl Interpreter {
    pub fn compile(program: String) -> Vec<Instruction> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        let mut instructions = Vec::new();
        let mut line_address = 0;

        for line in program.lines() {
            let class = Decoder::classify(line);

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, line_address);
                },
                LineClassification::Instruction(instruction) => {
                    let instruction = Decoder::decode(&instruction, &labels);
                    instructions.push(instruction);
                },
                LineClassification::Empty => {},
            }

            line_address += 4;
        }   
        
        instructions
    }
}
