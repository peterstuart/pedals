use crate::{audio_unit::AudioUnit, ring_buffer, Result};
use cpal::StreamConfig;
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::mpsc::{self, Receiver, Sender};
use Message::*;

pub type DelayMs = u32;

#[derive(Debug)]
pub enum Message {
    SetDelay(DelayMs),
}

pub struct Delay {
    delay_ms: DelayMs,
    stream_config: StreamConfig,
    messages: Receiver<Message>,
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Delay {
    pub fn new(
        stream_config: &StreamConfig,
        delay_ms: DelayMs,
        max_delay_ms: DelayMs,
    ) -> Result<(Self, Sender<Message>)> {
        let (sender, receiver) = mpsc::channel();

        // size the ring buffer so that it can accomodate the largest allowed delay
        let ring = RingBuffer::new(Self::delay_num_samples(stream_config, max_delay_ms) * 2);
        let (mut producer, consumer) = ring.split();

        ring_buffer::write_empty_samples(
            &mut producer,
            Self::delay_num_samples(stream_config, delay_ms),
        )?;

        Ok((
            Self {
                delay_ms,
                stream_config: stream_config.clone(),
                messages: receiver,
                producer,
                consumer,
            },
            sender,
        ))
    }

    fn delay_num_samples(stream_config: &StreamConfig, delay_ms: DelayMs) -> usize {
        let delay_num_frames = (delay_ms as f32 / 1_000.0) * stream_config.sample_rate.0 as f32;
        delay_num_frames as usize * stream_config.channels as usize
    }

    fn process_messages(&mut self) -> Result<()> {
        let messages: Vec<_> = self.messages.try_iter().collect();
        for message in messages {
            self.process_message(message)?;
        }

        Ok(())
    }

    fn process_message(&mut self, message: Message) -> Result<()> {
        match message {
            SetDelay(delay) => self.set_delay_ms(delay)?,
        };

        Ok(())
    }

    fn set_delay_ms(&mut self, delay_ms: DelayMs) -> Result<()> {
        let old_num_samples = Self::delay_num_samples(&self.stream_config, self.delay_ms);
        let new_num_samples = Self::delay_num_samples(&self.stream_config, delay_ms);

        if new_num_samples >= old_num_samples {
            ring_buffer::write_empty_samples(
                &mut self.producer,
                new_num_samples - old_num_samples,
            )?;
        } else {
            ring_buffer::read_samples(&mut self.consumer, old_num_samples - new_num_samples)?;
        }

        self.delay_ms = delay_ms;

        Ok(())
    }
}

impl AudioUnit for Delay {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        self.process_messages()?;

        ring_buffer::write_samples(&mut self.producer, input)?;
        let samples: Vec<f32> = ring_buffer::read_samples(&mut self.consumer, output.len())?;
        output.copy_from_slice(&samples);

        Ok(())
    }
}
