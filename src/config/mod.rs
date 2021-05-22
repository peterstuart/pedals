mod audio;
mod effect;
mod midi;

pub use effect::{DelayConfig, Effect};
pub use midi::{Midi, MidiSlider, NoteOn};

use crate::Result;
use audio::Audio;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub audio: Option<Audio>,
    pub midi: Option<Midi>,
    pub effects: Vec<Effect>,
}

impl Config {
    pub fn from(yaml: &str) -> Result<Config> {
        Ok(serde_yaml::from_str(yaml)?)
    }

    pub fn default() -> Self {
        Self {
            audio: None,
            midi: None,
            effects: vec![Effect::Transparent],
        }
    }
}
