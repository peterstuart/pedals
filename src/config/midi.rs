use serde::{Deserialize, Deserializer};
use std::{convert::TryInto, ops::Deref};
use wmidi::U7;

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
pub struct MidiSlider {
    pub channel: Channel,
    pub control_change: ControlChange,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct NoteOn {
    pub channel: Channel,
    pub note: Note,
}

impl NoteOn {
    pub fn new(channel: wmidi::Channel, note: wmidi::Note) -> Self {
        Self {
            channel: Channel::new(channel),
            note: Note::new(note),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Channel {
    #[serde(deserialize_with = "deserialize_channel")]
    value: wmidi::Channel,
}

impl Channel {
    pub fn new(channel: wmidi::Channel) -> Self {
        Self { value: channel }
    }
}

impl Deref for Channel {
    type Target = wmidi::Channel;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn deserialize_channel<'de, D>(deserializer: D) -> std::result::Result<wmidi::Channel, D::Error>
where
    D: Deserializer<'de>,
{
    let index: u8 = Deserialize::deserialize(deserializer)?;
    wmidi::Channel::from_index(index - 1).map_err(serde::de::Error::custom)
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct ControlChange {
    #[serde(deserialize_with = "deserialize_control_function")]
    value: wmidi::ControlFunction,
}

impl Deref for ControlChange {
    type Target = wmidi::ControlFunction;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn deserialize_control_function<'de, D>(
    deserializer: D,
) -> std::result::Result<wmidi::ControlFunction, D::Error>
where
    D: Deserializer<'de>,
{
    let value_u8: u8 = Deserialize::deserialize(deserializer)?;
    let value_u7: U7 = value_u8.try_into().map_err(serde::de::Error::custom)?;
    Ok(value_u7.into())
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Note {
    #[serde(deserialize_with = "deserialize_note")]
    value: wmidi::Note,
}

impl Note {
    pub fn new(note: wmidi::Note) -> Self {
        Self { value: note }
    }
}

impl Deref for Note {
    type Target = wmidi::Note;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn deserialize_note<'de, D>(deserializer: D) -> std::result::Result<wmidi::Note, D::Error>
where
    D: Deserializer<'de>,
{
    let value_u8: u8 = Deserialize::deserialize(deserializer)?;
    value_u8.try_into().map_err(serde::de::Error::custom)
}
