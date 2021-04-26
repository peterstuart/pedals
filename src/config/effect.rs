use crate::{
    audio::midi,
    audio_unit::{self, delay::DelayMs},
    config, AudioUnit, Pipeline, Result,
};
use cpal::StreamConfig;
use serde::Deserialize;
use wmidi::{Channel, ControlFunction, MidiMessage};
use Effect::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Effect {
    Transparent,
    Delay(Delay),
}

impl Effect {
    pub fn to_audio_unit(
        &self,
        midi_config: &Option<config::Midi>,
        stream_config: &StreamConfig,
    ) -> Result<audio_unit::Boxed> {
        Ok(match self {
            Transparent => audio_unit::Transparent::new().boxed(),
            Delay(delay_config) => delay_config.to_audio_unit(midi_config, stream_config)?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Delay {
    level: f32,
    delay_ms: u32,
    num: u32,
}

impl Delay {
    fn to_audio_unit(
        &self,
        midi_config: &Option<config::Midi>,
        stream_config: &StreamConfig,
    ) -> Result<audio_unit::Boxed> {
        let num = self.num;
        let channel = match midi_config {
            Some(midi_config) => midi_config.channel()?,
            None => config::Midi::default_channel(),
        };

        let mut audio_units = (1..=num)
            .map(|n| {
                let delay_unit = audio_unit::Delay::new(
                    stream_config,
                    Box::new(move |messages| Self::handle_midi_messages(channel, n, num, messages)),
                    self.delay_ms * n,
                )?
                .boxed();
                let gain_unit = audio_unit::Gain::new(self.level.powi(n as i32)).boxed();
                let pipeline = Pipeline::new(vec![delay_unit, gain_unit])?.boxed();

                Ok(pipeline)
            })
            .collect::<Result<Vec<_>>>()?;

        let transparent = audio_unit::Transparent::new().boxed();
        audio_units.insert(0, transparent);

        let split = audio_unit::Split::new(audio_units)?.boxed();

        Ok(split)
    }

    fn handle_midi_messages(
        channel: Channel,
        index: u32,
        num_delays: u32,
        messages: &[MidiMessage<'static>],
    ) -> Option<DelayMs> {
        let control_value =
            midi::latest_control_value(messages, channel, ControlFunction::MODULATION_WHEEL)?;

        let new_value = midi::interpolate_control_value(
            audio_unit::Delay::MIN_DELAY_MS,
            audio_unit::Delay::MAX_DELAY_MS / num_delays,
            control_value,
        ) * index;

        Some(new_value)
    }
}
