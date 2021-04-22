mod effect;

use crate::{pedal, Result};
use cpal::StreamConfig;
use effect::Effect;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pedals: Vec<Effect>,
}

impl Config {
    pub fn from(yaml: &str) -> Result<Config> {
        Ok(serde_yaml::from_str(yaml)?)
    }

    pub fn default() -> Self {
        Self {
            pedals: vec![Effect::Transparent],
        }
    }

    pub fn to_pedals(&self, stream_config: &StreamConfig) -> Result<Vec<pedal::Boxed>> {
        self.pedals
            .iter()
            .map(|pedal| pedal.to_pedal(stream_config))
            .collect()
    }
}
