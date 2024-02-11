use crate::midi::channel::MidiChannel;
use crate::midi::fractal::FractalMidiCC;
use crate::midi::program_change::AsU8;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlChange {
    pub channel: u8,
    pub control_number: u8,
    pub value: u8,
}

impl Default for ControlChange {
    fn default() -> Self {
        ControlChange {
            channel: 1,
            control_number: FractalMidiCC::SceneSelect as u8,
            value: 0,
        }
    }
}

impl Display for ControlChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CC: {:?}", self)
    }
}

impl AsU8 for ControlChange {
    fn as_u8(&self) -> Vec<u8> {
        vec![0xB0 + self.channel as u8, self.control_number, self.value]
    }
}
