use crate::midi::midi_message::MidiMessage;

#[derive(Debug, Clone, PartialEq)]
pub struct MidiBlock {
    pub id: usize,
    pub name: String,
    pub device_index: usize,
    pub messages: Vec<MidiMessage>,
}

impl MidiBlock {
    pub fn new(id: usize) -> MidiBlock {
        MidiBlock {
            id,
            name: "New MidiConfig".to_string(),
            device_index: 0,
            messages: Vec::new(),
        }
    }
}
