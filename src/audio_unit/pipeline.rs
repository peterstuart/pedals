use crate::{audio_unit, AudioUnit, Config, Result};
use anyhow::anyhow;
use cpal::StreamConfig;
use wmidi::MidiMessage;

pub struct Pipeline {
    audio_units: Vec<audio_unit::Boxed>,
}

impl Pipeline {
    pub fn from(config: &Config, stream_config: &StreamConfig) -> Result<Self> {
        let audio_units = config.to_audio_units(stream_config)?;
        Pipeline::new(audio_units)
    }

    pub fn new(audio_units: Vec<audio_unit::Boxed>) -> Result<Self> {
        if audio_units.is_empty() {
            Err(anyhow!("Must have at least one audio unit in the pipeline"))
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

    fn handle_midi_messages(&mut self, midi_messages: &[MidiMessage<'static>]) -> Result<()> {
        for audio_unit in &mut self.audio_units {
            audio_unit.handle_midi_messages(midi_messages)?;
        }

        Ok(())
    }
}
