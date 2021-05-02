use crate::config::NoteOn;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct LooperConfig {
    #[serde(default = "LooperConfig::default_loop_max")]
    pub max_ms: u32,
    pub toggle: NoteOn,
}

impl LooperConfig {
    const DEFAULT_LOOPER_MAX: u32 = 60_000;

    fn default_loop_max() -> u32 {
        Self::DEFAULT_LOOPER_MAX
    }
}
