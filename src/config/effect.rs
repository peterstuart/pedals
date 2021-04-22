use crate::{pedal, Pedal, Result};
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
    pub fn to_pedal(&self, stream_config: &StreamConfig) -> Result<pedal::Boxed> {
        Ok(match self {
            Transparent => pedal::Transparent::new().boxed(),
            Delay(delay_config) => delay_config.to_pedal(stream_config)?,
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
