use crate::{audio_unit::AudioUnit, ring_buffer, Result};
use anyhow::anyhow;
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
    pub const MIN_DELAY_MS: DelayMs = 0;
    pub const MAX_DELAY_MS: DelayMs = 10000;

    pub fn new(stream_config: &StreamConfig, delay_ms: DelayMs) -> Result<(Self, Sender<Message>)> {
        Self::validate_delay(delay_ms)?;

        let (sender, receiver) = mpsc::channel();

        // size the ring buffer so that it can accomodate the largest allowed delay
        let ring = RingBuffer::new(Self::delay_num_samples(stream_config, Self::MAX_DELAY_MS) * 2);
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

    fn validate_delay(delay_ms: DelayMs) -> Result<()> {
        if delay_ms > Self::MAX_DELAY_MS {
            return Err(anyhow!(
                "Delay must be <= {}, but was {}",
                Self::MAX_DELAY_MS,
                delay_ms
            ));
        } else {
            Ok(())
        }
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
        Self::validate_delay(delay_ms)?;

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

        ring_buffer::write_frame(&mut self.producer, input)?;
        let samples: Vec<f32> = ring_buffer::read_samples(&mut self.consumer, output.len())?;
        output.copy_from_slice(&samples);

        Ok(())
    }
}
