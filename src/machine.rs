mod decoder;

use std::collections::HashMap;
use eeric::prelude::*;

use decoder::Decoder;

use crate::machine::decoder::LineClassification;

pub struct RvMachine {
    core: RvCore
}

impl RvMachine {
    pub fn new() -> RvMachine {
        RvMachine {
            core: RvCore::new_zeroed()
        }
    }

    pub fn compile(&mut self, program: String) -> Vec<Instruction> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        let mut line_address = 0;

        for line in program.lines() {
            let class = Decoder::classify(line);

            match class {
                LineClassification::Label(label) => {
                    labels.insert(label, line_address);
                },
                LineClassification::Instruction(instruction) => {
                    Decoder::decode(&instruction, &labels);
                },
                LineClassification::Empty => {},
            }

            line_address += 4;
    }   


        todo!()
    }

    pub fn run(&mut self) -> impl Iterator<Item = RegistersSnapshot> + '_ {
        self.core.run()
    }
}
