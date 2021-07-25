pub mod number;

use cpal::StreamConfig;

pub fn zero_slice(data: &mut [f32]) {
    for value in data {
        *value = 0.0;
    }
}

pub fn ms_in_samples(stream_config: &StreamConfig, ms: u32) -> usize {
    let samples = (ms as f32 / 1_000.0) * stream_config.sample_rate.0 as f32;
    samples as usize * stream_config.channels as usize
}
