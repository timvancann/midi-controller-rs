use crate::midi::channel::MidiChannel;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub trait AsU8 {
    fn as_u8(&self) -> Vec<u8>;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramChange {
    pub channel: u8,
    pub program: u8,
}

impl Default for ProgramChange {
    fn default() -> Self {
        ProgramChange {
            channel: 1,
            program: 0,
        }
    }
}

impl Display for ProgramChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PC: {:?}", self)
    }
}

impl AsU8 for ProgramChange {
    fn as_u8(&self) -> Vec<u8> {
        vec![0xC0 + self.channel as u8, self.program]
    }
}
