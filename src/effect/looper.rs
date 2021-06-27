use crate::{
    audio::midi,
    audio_unit::{self, looper::Message, AudioUnit},
    config::LooperConfig,
    effect::Effect,
    Result,
};
use cpal::StreamConfig;
use std::sync::mpsc::Sender;
use wmidi::MidiMessage;

pub struct Looper {
    config: LooperConfig,
    split: audio_unit::Split,
    messages: Sender<Message>,
    num_clock_ticks: u32,
}

impl Looper {
    pub fn new(config: LooperConfig, stream_config: &StreamConfig) -> Result<Self> {
        let (looper, messages) = audio_unit::Looper::new(stream_config, config.max_ms);
        let transparent = audio_unit::Transparent::new();

        let split = audio_unit::Split::new(vec![looper.boxed(), transparent.boxed()])?;

        Ok(Self {
            config,
            split,
            messages,
            num_clock_ticks: 0,
        })
    }

    fn handle_midi_messages(&mut self, messages: &[midi::Message]) -> Result<()> {
        for message in messages {
            match message.message {
                MidiMessage::TimingClock => {
                    self.increment_clock_ticks();
                    if self.num_clock_ticks % (24 * 4) == 0 {
                        self.tick_measure()?;
                    }
                }
                MidiMessage::NoteOn(channel, note, _)
                    if channel == self.config.toggle.channel && note == self.config.toggle.note =>
                {
                    self.toggle()?;
                }
                MidiMessage::NoteOn(channel, note, _)
                    if channel == self.config.overdub.channel
                        && note == self.config.overdub.note =>
                {
                    self.enable_overdub_mode()?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn increment_clock_ticks(&mut self) {
        self.num_clock_ticks += 1;
    }

    fn tick_measure(&mut self) -> Result<()> {
        Ok(self.messages.send(Message::TickMeasure)?)
    }

    fn toggle(&mut self) -> Result<()> {
        Ok(self.messages.send(Message::Toggle)?)
    }

    fn enable_overdub_mode(&mut self) -> Result<()> {
        Ok(self.messages.send(Message::QueueOverdub)?)
    }
}

impl Effect for Looper {
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
