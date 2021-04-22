use crate::{audio_unit, util, AudioUnit, Result};
use anyhow::anyhow;

pub struct Split {
    audio_units: Vec<audio_unit::Boxed>,
}

impl Split {
    pub fn new(audio_units: Vec<audio_unit::Boxed>) -> Result<Self> {
        if audio_units.is_empty() {
            Err(anyhow!("Must have at least one audio unit in the Split"))
        } else {
            Ok(Self { audio_units })
        }
    }
}

impl AudioUnit for Split {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        let length = output.len();
        util::zero_slice(output);

        let mut audio_unit_output: Vec<f32> = vec![0.0; length];

        for audio_unit in &mut self.audio_units {
            audio_unit.process(input, &mut audio_unit_output)?;

            for i in 0..output.len() {
                output[i] += audio_unit_output[i];
            }
        }

        Ok(())
    }
}
