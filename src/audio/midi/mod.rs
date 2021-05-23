mod message;

pub use message::Message;

use crate::{config::MidiSlider, Result};
use anyhow::anyhow;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use num_traits::Num;
use std::sync::mpsc::Sender;
use std::sync::mpsc::{self, Receiver};
use wmidi::{ControlValue, MidiMessage};

/// Listens on the provided port, and sends MIDI messages over a channel. Returns the receiver of that channel.
pub fn listen_for_input(port_name: &str) -> Result<(Receiver<Message>, MidiInputConnection<()>)> {
    let port = port(port_name)?;

    println!("MIDI input: {}", port_name);

    let (sender, receiver) = mpsc::channel();
    let midi_input = handle_messages(port, sender)?;

    Ok((receiver, midi_input))
}

pub fn port_names() -> Result<Vec<String>> {
    let midi_input = midi_input()?;
    midi_input
        .ports()
        .iter()
        .map(|port| midi_input.port_name(port).map_err(|e| e.into()))
        .collect()
}

/// Get the last value for a `ControlFunction` from a list of messages.
pub fn latest_control_value(slider: MidiSlider, messages: &[Message]) -> Option<ControlValue> {
    messages
        .iter()
        .rev()
        .find_map(|message| match message.message {
            MidiMessage::ControlChange(ch, function, value)
                if ch == *slider.channel && function == *slider.control_change =>
            {
                Some(value)
            }

            _ => None,
        })
}

/// Maps a control value (0-127) into a range.
/// eg. `control_value_in_range(50, 100, 64) == 75`
pub fn interpolate_control_value<T: Num + From<u8> + Copy>(
    min: T,
    max: T,
    value: ControlValue,
) -> T {
    let control_value_min: T = from_control_value(ControlValue::MIN);
    let control_value_max: T = from_control_value(ControlValue::MAX);

    let value: T = from_control_value(value);
    (value - control_value_min) * (max - min) / (control_value_max - control_value_min) + min
}

fn midi_input() -> Result<MidiInput> {
    Ok(MidiInput::new("Input")?)
}

fn port(name: &str) -> Result<MidiInputPort> {
    let names = port_names()?;
    let midi_input = midi_input()?;

    midi_input
        .ports()
        .into_iter()
        .find(|port| midi_input.port_name(&port) == Ok(name.into()))
        .ok_or_else(|| {
            anyhow!(
                "Could not find a MIDI port with name '{}'. Available ports are:\n{}",
                name,
                names.join("\n")
            )
        })
}

fn handle_messages(
    port: MidiInputPort,
    sender: Sender<Message>,
) -> Result<MidiInputConnection<()>> {
    Ok(midi_input()?
        .connect(
            &port,
            "midir-read-input",
            move |timestamp, bytes, _| {
                if let Some(message) = Message::from(timestamp, bytes).unwrap() {
                    sender.send(message).unwrap();
                }
            },
            (),
        )
        .map_err(|e| anyhow!("could not connect: {}", e))?)
}

fn from_control_value<T: From<u8>>(value: ControlValue) -> T {
    let byte: u8 = value.into();
    byte.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_from_control_value() {
        assert_eq!(from_control_value::<u32>(0_u8.try_into().unwrap()), 0);
        assert_eq!(from_control_value::<u32>(1_u8.try_into().unwrap()), 1);
        assert_eq!(from_control_value::<u32>(127_u8.try_into().unwrap()), 127);
    }

    #[test]
    fn test_interpolate_control_value() {
        assert_eq!(
            interpolate_control_value(0_u32, 100_u32, 0_u8.try_into().unwrap()),
            0
        );
        assert_eq!(
            interpolate_control_value(0_u32, 100_u32, 127_u8.try_into().unwrap()),
            100
        );
        assert_eq!(
            interpolate_control_value(0_u32, 100_u32, 64_u8.try_into().unwrap()),
            50
        );

        assert_eq!(
            interpolate_control_value(50_u32, 100_u32, 0_u8.try_into().unwrap()),
            50
        );
        assert_eq!(
            interpolate_control_value(50_u32, 100_u32, 127_u8.try_into().unwrap()),
            100
        );
        assert_eq!(
            interpolate_control_value(50_u32, 100_u32, 64_u8.try_into().unwrap()),
            75
        );
    }
}
