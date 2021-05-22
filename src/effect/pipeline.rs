use cpal::StreamConfig;

use crate::{
    audio::midi::Message,
    effect::{self, Effect},
    Config, Result,
};

pub struct Pipeline {
    effects: Vec<effect::Boxed>,
}

impl Pipeline {
    pub fn from(config: &Config, stream_config: &StreamConfig) -> Result<Self> {
        let effects = config
            .effects
            .iter()
            .map(|effect_config| effect::from(*effect_config, stream_config))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { effects })
    }

    pub fn new(effects: Vec<effect::Boxed>) -> Self {
        Self { effects }
    }
}

impl Effect for Pipeline {
    fn process(
        &mut self,
        midi_messages: &[Message],
        input: &[f32],
        output: &mut [f32],
    ) -> Result<()> {
        let mut input = input.to_vec();

        for effect in &mut self.effects {
            effect.process(midi_messages, &input, output)?;
            input.copy_from_slice(output);
        }

        Ok(())
    }
}
