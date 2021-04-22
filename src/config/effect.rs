use crate::{audio_unit, AudioUnit, Result};
use cpal::StreamConfig;
use serde::Deserialize;
use Effect::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(DelayConfig),
}

impl Effect {
    pub fn to_audio_unit(&self, stream_config: &StreamConfig) -> Result<audio_unit::Boxed> {
        Ok(match self {
            Transparent => audio_unit::Transparent::new().boxed(),
            Delay(delay_config) => delay_config.to_audio_unit(stream_config)?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct DelayConfig {
    level: f32,
    delay_ms: u32,
    num: u32,
}

impl DelayConfig {
    fn to_audio_unit(&self, stream_config: &StreamConfig) -> Result<audio_unit::Boxed> {
        let mut audio_units = (1..=self.num)
            .map(|n| {
                Ok(audio_unit::Delay::new(
                    stream_config,
                    self.level.powi(n as i32),
                    self.delay_ms * n,
                )?
                .boxed())
            })
            .collect::<Result<Vec<_>>>()?;

        let transparent = audio_unit::Transparent::new().boxed();
        audio_units.insert(0, transparent);

        Ok(audio_unit::Split::new(audio_units)?.boxed())
    }
}
