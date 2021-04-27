use crate::{
    audio::midi,
    audio_unit::{self, delay::Message, AudioUnit},
    config::DelayConfig,
    effect::Effect,
    Result,
};
use cpal::StreamConfig;
use std::sync::mpsc::Sender;
use wmidi::MidiMessage;

pub struct Delay {
    config: DelayConfig,
    split: audio_unit::Split,
    message_senders: Vec<Sender<Message>>,
}

impl Delay {
    pub fn new(config: DelayConfig, stream_config: &StreamConfig) -> Result<Self> {
        let mut audio_units = vec![];
        let mut message_senders = vec![];

        for n in 0..config.num {
            let delay = Self::delay_for_index(config.delay_ms, n);
            let max_total_delay = config.max_delay_ms * config.num;
            let (delay_unit, messages) =
                audio_unit::Delay::new(stream_config, delay, max_total_delay)?;
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
            split,
            message_senders,
        })
    }

    fn handle_midi_messages(&mut self, messages: &[MidiMessage<'static>]) -> Result<()> {
        if let Some(delay) = self.delay_from_midi_messages(messages) {
            self.set_delay(delay)?;
        }

        Ok(())
    }

    fn delay_from_midi_messages(&self, messages: &[MidiMessage<'static>]) -> Option<u32> {
        let midi_slider = self.config.delay_ms_slider?;
        let control_value = midi::latest_control_value(midi_slider, messages)?;
        let new_value = midi::interpolate_control_value(0, self.config.max_delay_ms, control_value);

        Some(new_value)
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
        midi_messages: &[MidiMessage<'static>],
        input: &[f32],
        output: &mut [f32],
    ) -> Result<()> {
        self.handle_midi_messages(midi_messages)?;
        self.split.process(input, output)
    }
}
