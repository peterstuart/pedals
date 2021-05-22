use super::tap_tempo::TapTempo;
use crate::{
    audio::midi,
    audio_unit::{self, delay::Message, AudioUnit},
    config::DelayConfig,
    effect::Effect,
    Result,
};
use anyhow::anyhow;
use cpal::StreamConfig;
use std::sync::mpsc::Sender;

pub struct Delay {
    config: DelayConfig,
    tap_tempo: Option<TapTempo>,
    split: audio_unit::Split,
    message_senders: Vec<Sender<Message>>,
}

impl Delay {
    pub fn new(config: DelayConfig, stream_config: &StreamConfig) -> Result<Self> {
        Self::validate_config(&config)?;

        let tap_tempo = config.tap_tempo.map(TapTempo::new);

        let mut audio_units = vec![];
        let mut message_senders = vec![];

        for n in 0..config.num {
            let delay = Self::delay_for_index(config.delay_ms, n);
            let max_delay = config.max_delay_ms * (n + 1);
            let (delay_unit, messages) = audio_unit::Delay::new(stream_config, delay, max_delay)?;
            let gain_unit = audio_unit::Gain::new(config.level.powi(n as i32)).boxed();
            let pipeline = audio_unit::Pipeline::new(vec![delay_unit.boxed(), gain_unit])?.boxed();

            audio_units.insert(audio_units.len(), pipeline);
            message_senders.insert(message_senders.len(), messages);
        }

        let transparent = audio_unit::Transparent::new().boxed();
        audio_units.insert(audio_units.len(), transparent);

        let split = audio_unit::Split::new(audio_units)?;

        Ok(Self {
            config,
            tap_tempo,
            split,
            message_senders,
        })
    }

    fn validate_config(config: &DelayConfig) -> Result<()> {
        if config.delay_ms * config.num <= config.max_delay_ms {
            Ok(())
        } else {
            Err(anyhow!("Invalid delay config: {:#?}", config))
        }
    }

    fn handle_midi_messages(&mut self, messages: &[midi::Message]) -> Result<()> {
        if let Some(delay) = self.delay_from_midi_messages(messages) {
            self.set_delay(delay)?;
        }

        Ok(())
    }

    fn delay_from_midi_messages(&mut self, messages: &[midi::Message]) -> Option<u32> {
        let delay_from_slider = self.delay_from_midi_messages_slider(messages);
        let delay_from_tap_tempo = self.delay_from_midi_messages_tap(messages);

        delay_from_slider.or(delay_from_tap_tempo)
    }

    fn delay_from_midi_messages_slider(&self, messages: &[midi::Message]) -> Option<u32> {
        let midi_slider = self.config.delay_ms_slider?;
        let control_value = midi::latest_control_value(midi_slider, messages)?;
        let new_value = midi::interpolate_control_value(
            self.config.min_delay_ms,
            self.config.max_delay_ms,
            control_value,
        );

        Some(new_value)
    }

    fn delay_from_midi_messages_tap(&mut self, messages: &[midi::Message]) -> Option<u32> {
        let tap_tempo = self.tap_tempo.as_mut()?;
        let tempo = tap_tempo.handle_messages(messages)?;

        Some(tempo.beat_duration_as_ms())
    }

    fn set_delay(&mut self, delay_ms: u32) -> Result<()> {
        for (i, sender) in self.message_senders.iter().enumerate() {
            let delay_ms = Self::delay_for_index(delay_ms, i as u32);
            let message = Message::SetDelay(delay_ms);
            sender.send(message)?;
        }

        Ok(())
    }

    fn delay_for_index(base_delay: u32, index: u32) -> u32 {
        base_delay * (index + 1)
    }
}

impl Effect for Delay {
    fn process(
        &mut self,
        midi_messages: &[midi::Message],
        input: &[f32],
        output: &mut [f32],
    ) -> Result<()> {
        self.handle_midi_messages(midi_messages)?;
        self.split.process(input, output)
    }
}
