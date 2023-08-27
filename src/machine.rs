mod decoder;

use decoder::Decoder;

pub struct RvMachine {
    core: eeric::RvCore,
    decoder: Decoder
}

impl RvMachine {
    pub fn new() -> RvMachine {
        RvMachine {
            core: eeric::RvCore::new_zeroed(),
            decoder: Decoder
        }
    }


}