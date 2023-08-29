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


        // todo: separate into 2 loops, first one for all labels, second one for instructions
        for line in program.lines() {
            let class = Decoder::classify(line);

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, line_address);
                },
                LineClassification::Instruction(instruction) => {
                    let maybe_instruction = Decoder::decode(&instruction, &labels, line_address);

                    match maybe_instruction {
                        Ok(instruction) => instructions.push(instruction),
                        Err(msg) => errors.push((line_address, msg)) 
                    };
                },
                LineClassification::Empty => {},
            }

            line_address += 4;
        }   
        
        if errors.is_empty() {
            Ok(instructions)
        } else {
            Err(errors)
        }
    }
}
