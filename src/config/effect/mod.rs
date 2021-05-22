mod delay;

pub use delay::DelayConfig;

use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(DelayConfig),
}
