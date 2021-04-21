use crate::{ring_buffer, Pedal, Result};
use cpal::StreamConfig;
use ringbuf::{Consumer, Producer, RingBuffer};

pub struct Delay {
    level: f32,
    producer: Producer<f32>,
    consumer: Consumer<f32>,
}

impl Delay {
    pub fn new(stream_config: &StreamConfig, level: f32, delay_ms: u32) -> Result<Self> {
        let delay_num_frames = (delay_ms as f32 / 1_000.0) * stream_config.sample_rate.0 as f32;
        let delay_num_samples = delay_num_frames as usize * stream_config.channels as usize;

        let ring = RingBuffer::new(delay_num_samples * 2);
        let (mut producer, consumer) = ring.split();

        ring_buffer::write_empty_samples(&mut producer, delay_num_samples)?;

        Ok(Self {
            level,
            producer,
            consumer,
        })
    }
}

impl Pedal for Delay {
    fn name(&self) -> String {
        "Delay".into()
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        ring_buffer::write_frame(&mut self.producer, input)?;
        let samples: Vec<f32> = ring_buffer::read_frame(&mut self.consumer, output.len())?
            .into_iter()
            .map(|sample| sample * self.level)
            .collect();

        output.copy_from_slice(&samples);

        Ok(())
    }
}
