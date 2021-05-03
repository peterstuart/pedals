use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Audio {
    #[serde(default = "Audio::default_latency_ms")]
    pub latency_ms: u32,
    pub input: Option<String>,
    pub output: Option<String>,
}

impl Audio {
    const DEFAULT_LATENCY_MS: u32 = 50;

    fn default_latency_ms() -> u32 {
        Self::DEFAULT_LATENCY_MS
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            latency_ms: Self::default_latency_ms(),
            input: None,
            output: None,
        }
    }
}
