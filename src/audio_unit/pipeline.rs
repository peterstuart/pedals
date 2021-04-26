use crate::{
    audio_unit::{self, AudioUnit},
    Result,
};
use anyhow::anyhow;

pub struct Pipeline {
    audio_units: Vec<audio_unit::Boxed>,
}

impl Pipeline {
    pub fn new(audio_units: Vec<audio_unit::Boxed>) -> Result<Self> {
        if audio_units.is_empty() {
            Err(anyhow!("Must have at least one audio unit in the Pipeline"))
        } else {
            Ok(Self { audio_units })
        }
    }
}

impl AudioUnit for Pipeline {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        let mut input = input.to_vec();

        for audio_unit in &mut self.audio_units {
            audio_unit.process(&input, output)?;
            input.copy_from_slice(output);
        }

        Ok(())
    }
}
