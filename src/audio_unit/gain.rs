use crate::{audio_unit::AudioUnit, Result};

pub struct Gain {
    gain: f32,
}

impl Gain {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl AudioUnit for Gain {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        for i in 0..input.len() {
            output[i] = input[i] * self.gain;
        }

        Ok(())
    }
}
