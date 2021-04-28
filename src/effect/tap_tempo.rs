use crate::{audio::midi::Message, config::NoteOn, effect::Tempo};
use wmidi::MidiMessage;
use State::*;

#[derive(Debug)]
enum State {
    Start,
    TappedOnce(u64),
}

#[derive(Debug)]
pub struct TapTempo {
    note_on: NoteOn,
    state: State,
}

impl TapTempo {
    const MAX_DELAY_BETWEEN_TAPS: u64 = 2_000_000; // microseconds

    pub fn new(note_on: NoteOn) -> Self {
        Self {
            note_on,
            state: State::Start,
        }
    }

    pub fn handle_messages(&mut self, messages: &[Message]) -> Option<Tempo> {
        messages
            .iter()
            .flat_map(|message| self.handle_message(message))
            .last()
    }

    fn handle_message(&mut self, message: &Message) -> Option<Tempo> {
        match message.message {
            MidiMessage::NoteOn(channel, note, _)
                if channel == *self.note_on.channel && note == *self.note_on.note =>
            {
                self.handle_tap(message.timestamp)
            }
            _ => None,
        }
    }

    fn handle_tap(&mut self, timestamp: u64) -> Option<Tempo> {
        match self.state {
            Start => {
                self.state = TappedOnce(timestamp);
                None
            }
            TappedOnce(last_tap) => {
                let time_since_last_tap = timestamp - last_tap;

                if time_since_last_tap <= Self::MAX_DELAY_BETWEEN_TAPS {
                    self.state = TappedOnce(timestamp);
                    Some(Tempo::new(timestamp, time_since_last_tap))
                } else {
                    self.state = TappedOnce(timestamp);
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wmidi::{Channel, Note, Velocity};

    fn test_handle_messages_with(messages: &[Message], expected_tempo: Option<Tempo>) {
        let note_on = NoteOn::new(Channel::Ch1, Note::A0);
        let mut tap_tempo = TapTempo::new(note_on);
        let tempo = tap_tempo.handle_messages(&messages);

        assert_eq!(tempo, expected_tempo);
    }

    #[test]
    fn test_handle_messages_two_note_ons() {
        test_handle_messages_with(
            &vec![
                Message::new(
                    0,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    500_000,
                    MidiMessage::NoteOff(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    1_000_000,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
            ],
            Some(Tempo::new(1_000_000, 1_000_000)),
        );
    }

    #[test]
    fn test_handle_messages_three_note_ons() {
        // three note-ons
        test_handle_messages_with(
            &vec![
                Message::new(
                    0,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    500_000,
                    MidiMessage::NoteOff(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    1_000_000,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    1_250_000,
                    MidiMessage::NoteOff(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    1_500_000,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
            ],
            Some(Tempo::new(1_500_000, 500_000)),
        );
    }

    #[test]
    fn test_handle_messages_two_note_ons_too_distant() {
        // two note-ons
        test_handle_messages_with(
            &vec![
                Message::new(
                    0,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    500_000,
                    MidiMessage::NoteOff(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
                Message::new(
                    3_000_000,
                    MidiMessage::NoteOn(Channel::Ch1, Note::A0, Velocity::MAX),
                ),
            ],
            None,
        );
    }
}
