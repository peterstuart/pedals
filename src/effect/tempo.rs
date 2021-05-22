#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tempo {
    start_timestamp: u64,
    beat_duration: u64,
}

impl Tempo {
    pub fn new(start_timestamp: u64, beat_duration: u64) -> Self {
        Self {
            start_timestamp,
            beat_duration,
        }
    }

    pub fn beat_duration_as_ms(&self) -> u32 {
        (self.beat_duration / 1000) as u32
    }
}
