use crate::{audio_unit::AudioUnit, util::number, Result};
use rustfft::FftPlanner;

pub struct Fft {
    planner: FftPlanner<f32>,
}

impl Fft {
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
        }
    }
}

impl AudioUnit for Fft {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        let mut input_as_complex: Vec<_> = input.iter().copied().map(number::to_complex).collect();

        let length = input.len();

        self.planner
            .plan_fft_forward(length)
            .process(&mut input_as_complex);

        self.planner
            .plan_fft_inverse(length)
            .process(&mut input_as_complex);

        let scale = length as f32;

        for i in 0..length {
            // the imaginary part of the number is ~0, so it can be discarded
            output[i] = input_as_complex[i].re / scale;
        }

        Ok(())
    }
}

impl Default for Fft {
    fn default() -> Self {
        Self::new()
    }
}
