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
            Delay(delay_config) => delay_config.to_pedal(stream_config)?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct DelayConfig {
    pub level: f32,
    pub delay_ms: u32,
    pub num: u32,
}

impl DelayConfig {
    fn to_pedal(&self, stream_config: &StreamConfig) -> Result<pedal::Boxed> {
        let mut pedals = (1..=self.num)
            .map(|n| {
                Ok(
                    pedal::Delay::new(stream_config, self.level.powi(n as i32), self.delay_ms * n)?
                        .boxed(),
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let transparent = pedal::Transparent::new().boxed();
        pedals.insert(0, transparent);

        Ok(pedal::Split::new(pedals)?.boxed())
    }
}
