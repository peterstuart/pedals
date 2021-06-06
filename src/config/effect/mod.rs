mod delay;
mod looper;

pub use delay::DelayConfig;
pub use looper::LooperConfig;

use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(DelayConfig),
    Looper(LooperConfig),
}
