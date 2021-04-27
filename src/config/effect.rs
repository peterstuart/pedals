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
    pub level: f32,
    pub delay_ms: DelayMs,
    pub num: u32,
    pub delay_ms_slider: Option<MidiSlider>,
}
