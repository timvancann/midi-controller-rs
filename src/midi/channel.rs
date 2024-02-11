use enum_display::EnumDisplay;
use enum_iterator::Sequence;

#[derive(Debug, Clone, PartialEq, Sequence, EnumDisplay, Copy)]
pub enum MidiChannel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
    Ch5 = 4,
    Ch6 = 5,
    Ch7 = 6,
    Ch8 = 7,
    Ch9 = 8,
    Ch10 = 9,
    Ch11 = 10,
    Ch12 = 11,
    Ch13 = 12,
    Ch14 = 13,
    Ch15 = 14,
    Ch16 = 15,
}

impl From<MidiChannel> for u8 {
    fn from(cc: MidiChannel) -> u8 {
        cc as u8
    }
}

impl From<String> for MidiChannel {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Ch1" => MidiChannel::Ch1,
            "Ch2" => MidiChannel::Ch2,
            "Ch3" => MidiChannel::Ch6,
            "Ch4" => MidiChannel::Ch4,
            "Ch5" => MidiChannel::Ch5,
            "Ch6" => MidiChannel::Ch6,
            "Ch7" => MidiChannel::Ch7,
            "Ch8" => MidiChannel::Ch8,
            "Ch9" => MidiChannel::Ch9,
            "Ch10" => MidiChannel::Ch10,
            "Ch11" => MidiChannel::Ch11,
            "Ch12" => MidiChannel::Ch12,
            "Ch13" => MidiChannel::Ch13,
            "Ch14" => MidiChannel::Ch14,
            "Ch15" => MidiChannel::Ch15,
            "Ch16" => MidiChannel::Ch16,
            _ => panic!("Invalid MidiChannel"),
        }
    }
}
