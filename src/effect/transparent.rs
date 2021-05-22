use crate::{
    audio::midi::Message,
    audio_unit::{self, AudioUnit},
    effect::Effect,
    Result,
};

pub struct Transparent {
    unit: audio_unit::Transparent,
}

impl Transparent {
    pub fn new() -> Self {
        Self {
            unit: audio_unit::Transparent::new(),
        }
    }
}

impl Effect for Transparent {
    fn process(&mut self, _: &[Message], input: &[f32], output: &mut [f32]) -> Result<()> {
        self.unit.process(input, output)
    }
}

impl Default for Transparent {
    fn default() -> Self {
        Self::new()
    }
}
