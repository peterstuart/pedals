use crate::{audio_unit, AudioUnit, Pipeline, Result};
use cpal::StreamConfig;
use serde::Deserialize;
use Effect::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(Delay),
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
pub struct Delay {
    level: f32,
    delay_ms: u32,
    num: u32,
}

impl Delay {
    fn to_audio_unit(&self, stream_config: &StreamConfig) -> Result<audio_unit::Boxed> {
        let mut audio_units = (1..=self.num)
            .map(|n| {
                Ok(Pipeline::new(vec![
                    audio_unit::Delay::new(stream_config, self.delay_ms * n)?.boxed(),
                    audio_unit::Gain::new(self.level.powi(n as i32)).boxed(),
                ])?
                .boxed())
            })
            .collect::<Result<Vec<_>>>()?;

        let transparent = audio_unit::Transparent::new().boxed();
        audio_units.insert(0, transparent);

        Ok(audio_unit::Split::new(audio_units)?.boxed())
    }
}
