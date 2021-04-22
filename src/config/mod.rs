mod effect;

use crate::{audio_unit, Result};
use cpal::StreamConfig;
use effect::Effect;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    effects: Vec<Effect>,
}

impl Config {
    pub fn from(yaml: &str) -> Result<Config> {
        Ok(serde_yaml::from_str(yaml)?)
    }

    pub fn default() -> Self {
        Self {
            effects: vec![Effect::Transparent],
        }
    }

    pub fn to_audio_units(&self, stream_config: &StreamConfig) -> Result<Vec<audio_unit::Boxed>> {
        self.effects
            .iter()
            .map(|audio_unit| audio_unit.to_audio_unit(stream_config))
            .collect()
    }
}
