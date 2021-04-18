use crate::{pedal, Pedal as PedalTrait, Result};
use cpal::StreamConfig;
use serde::Deserialize;
use Pedal::*;

#[derive(Debug, Deserialize)]
pub struct Config {
    pedals: Vec<Pedal>,
}

impl Config {
    pub fn from(yaml: &str) -> Result<Config> {
        Ok(serde_yaml::from_str(yaml)?)
    }

    pub fn default() -> Self {
        Self {
            pedals: vec![Transparent],
        }
    }

    pub fn to_pedals(&self, stream_config: &StreamConfig) -> Result<Vec<pedal::Boxed>> {
        self.pedals
            .iter()
            .map(|pedal| pedal.to_pedal(stream_config))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Pedal {
    Transparent,
    Delay(DelayConfig),
}

impl Pedal {
    fn to_pedal(&self, stream_config: &StreamConfig) -> Result<pedal::Boxed> {
        Ok(match self {
            Transparent => pedal::Transparent::new().boxed(),
            Delay(delay_config) => pedal::Delay::new(stream_config, delay_config)?.boxed(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct DelayConfig {
    pub level: f32,
    pub delay_ms: u32,
}
