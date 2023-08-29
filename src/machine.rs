mod decoder;

use std::collections::HashMap;

use decoder::Decoder;

pub struct RvMachine {
    core: eeric::RvCore
}

impl RvMachine {
    pub fn new() -> RvMachine {
        RvMachine {
            core: eeric::RvCore::new_zeroed()
        }
    }

    pub fn compile(&mut self, program: String) -> Vec<eeric::Instruction> {
        let mut labels: HashMap<&str, u64> = HashMap::new();
        let mut line_address = 0;

        for line in program.lines() {
            let mut tokens = line.split_whitespace();
            let first_token = tokens.next().unwrap();
            if first_token.ends_with(":") {
                let label = first_token.trim_end_matches(":");
                labels.insert(label, line_address);
            }
            line_address += 4;
        }

        let mut class = Decoder::classify(instruction);
        decoder.decode()
    }
}
