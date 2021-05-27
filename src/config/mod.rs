mod audio;
mod effect;
mod midi;

pub use audio::Audio;
pub use effect::{DelayConfig, Effect};
pub use midi::{Midi, MidiSlider, NoteOn};

use crate::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub audio: Audio,
    #[serde(default)]
    pub midi: Midi,
    pub effects: Vec<Effect>,
}

impl Config {
    pub fn from(yaml: &str) -> Result<Config> {
        Ok(serde_yaml::from_str(yaml)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: Audio::default(),
            midi: Midi::default(),
            effects: vec![Effect::Transparent],
        }
    }
}
