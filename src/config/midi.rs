use serde::{Deserialize, Deserializer};
use std::convert::TryInto;
use wmidi::{Channel, ControlFunction, Note, U7};

#[derive(Debug, Deserialize)]
pub struct Midi {
    pub port: Option<String>,
}

impl Default for Midi {
    fn default() -> Self {
        Self { port: None }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct NoteOn {
    #[serde(deserialize_with = "deserialize_channel")]
    pub channel: Channel,
    #[serde(deserialize_with = "deserialize_note")]
    pub note: Note,
}

impl NoteOn {
    pub fn new(channel: Channel, note: Note) -> Self {
        Self { channel, note }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MidiSlider {
    #[serde(deserialize_with = "deserialize_channel")]
    pub channel: Channel,
    #[serde(deserialize_with = "deserialize_control_function")]
    pub control_change: ControlFunction,
}

fn deserialize_channel<'de, D>(deserializer: D) -> std::result::Result<Channel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u8 = Deserialize::deserialize(deserializer)?;
    wmidi::Channel::from_index(index - 1).map_err(serde::de::Error::custom)
}

fn deserialize_control_function<'de, D>(
    deserializer: D,
) -> std::result::Result<ControlFunction, D::Error>
where
    D: Deserializer<'de>,
{
    let value_u8: u8 = Deserialize::deserialize(deserializer)?;
    let value_u7: U7 = value_u8.try_into().map_err(serde::de::Error::custom)?;
    Ok(value_u7.into())
}

fn deserialize_note<'de, D>(deserializer: D) -> std::result::Result<wmidi::Note, D::Error>
where
    D: Deserializer<'de>,
{
    let value_u8: u8 = Deserialize::deserialize(deserializer)?;
    value_u8.try_into().map_err(serde::de::Error::custom)
}
