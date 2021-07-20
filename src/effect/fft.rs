use crate::{
    audio::midi::Message,
    audio_unit::{self, AudioUnit},
    effect::Effect,
    Result,
};

pub struct Fft {
    unit: audio_unit::Fft,
}

impl Fft {
    pub fn new() -> Self {
        Self {
            unit: audio_unit::Fft::new(),
        }
    }
}

impl Effect for Fft {
    fn process(&mut self, _: &[Message], input: &[f32], output: &mut [f32]) -> Result<()> {
        self.unit.process(input, output)
    }
}

impl Default for Fft {
    fn default() -> Self {
        Self::new()
    }
}
