use std::error::Error;
use std::fmt::{Display, Formatter};
use std::thread::sleep;

use midir::{MidiOutput, MidiOutputConnection};
use serde::{Deserialize, Serialize};

use crate::midi::control_change::ControlChange;
use crate::midi::program_change::{AsU8, ProgramChange};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MidiMessage {
    Delay(u64),
    Empty,
    ProgramChange(ProgramChange),
    ControlChange(ControlChange),
}

impl MidiMessage {
    pub fn all() -> Vec<MidiMessage> {
        vec![
            MidiMessage::Empty,
            MidiMessage::Delay(0),
            MidiMessage::ProgramChange(ProgramChange::default()),
            MidiMessage::ControlChange(ControlChange::default()),
        ]
    }
}

impl Default for MidiMessage {
    fn default() -> Self {
        MidiMessage::Empty
    }
}

impl Display for MidiMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MidiMessage::Empty => write!(f, "Empty"),
            MidiMessage::Delay(delay) => write!(f, "Delay: {} ms", delay),
            MidiMessage::ProgramChange(pc) => write!(f, "{:?}", pc),
            MidiMessage::ControlChange(cc) => write!(f, "{:?}", cc),
        }
    }
}

impl From<&str> for MidiMessage {
    fn from(message: &str) -> Self {
        match message {
            "ProgramChange" => MidiMessage::ProgramChange(ProgramChange::default()),
            "ControlChange" => MidiMessage::ControlChange(ControlChange::default()),
            _ => MidiMessage::Empty,
        }
    }
}
pub fn send_midi_messages(device: usize, midi_messages: Vec<MidiMessage>) {
    for message in midi_messages {
        match message {
            MidiMessage::Empty => (),
            MidiMessage::Delay(delay) => sleep(std::time::Duration::from_millis(delay)),
            MidiMessage::ProgramChange(pc) => {
                match send_midi_message(device, pc.as_u8().as_slice()) {
                    Ok(_) => log::info!("Program change sent successfully"),
                    Err(err) => log::error!("Error sending program change: {}", err),
                }
            }
            MidiMessage::ControlChange(cc) => {
                match send_midi_message(device, cc.as_u8().as_slice()) {
                    Ok(_) => log::info!("Control change sent successfully"),
                    Err(err) => log::error!("Error sending program change: {}", err),
                }
            }
        }
    }
}

fn connect_to_midi_device(device_index: usize) -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("My MIDI Output")?;
    let out_ports = midi_out.ports();
    if device_index >= out_ports.len() {
        return Err("Invalid device index".into());
    }
    Ok(midi_out.connect(&out_ports[device_index], "midir-test")?)
}

fn send_midi_message(
    device_index: usize,
    message: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn_out = connect_to_midi_device(device_index)?;
    conn_out.send(message)?;
    conn_out.close();
    Ok(())
}
