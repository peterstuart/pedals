use crate::{audio_unit::delay::DelayMs, config::MidiSlider};
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(DelayConfig),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DelayConfig {
    #[serde(default = "DelayConfig::default_level")]
    pub level: f32,
    #[serde(default = "DelayConfig::default_delay")]
    pub delay_ms: DelayMs,
    #[serde(default = "DelayConfig::default_delay_min")]
    pub min_delay_ms: DelayMs,
    #[serde(default = "DelayConfig::default_delay_max")]
    pub max_delay_ms: DelayMs,
    #[serde(default = "DelayConfig::default_num")]
    pub num: u32,
    pub delay_ms_slider: Option<MidiSlider>,
}

impl DelayConfig {
    const DEFAULT_LEVEL: f32 = 0.5;
    const DEFAULT_DELAY: DelayMs = 250;
    const DEFAULT_DELAY_MIN: DelayMs = 0;
    const DEFAULT_DELAY_MAX: DelayMs = 2000;
    const DEFAULT_NUM: u32 = 6;

    fn default_level() -> f32 {
        Self::DEFAULT_LEVEL
    }

    fn default_delay() -> DelayMs {
        Self::DEFAULT_DELAY
    }

    fn default_delay_min() -> DelayMs {
        Self::DEFAULT_DELAY_MIN
    }

    fn default_delay_max() -> DelayMs {
        Self::DEFAULT_DELAY_MAX
    }

    fn default_num() -> u32 {
        Self::DEFAULT_NUM
    }
}
