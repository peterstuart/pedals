use crate::Result;
use anyhow::anyhow;
use ringbuf::{Consumer, Producer};

pub fn write_empty_samples(producer: &mut Producer<f32>, size: usize) -> Result<()> {
    let data = vec![0.0; size];
    write_samples(producer, &data)
}

pub fn write_samples(producer: &mut Producer<f32>, data: &[f32]) -> Result<()> {
    let wrote = producer.push_slice(data);
    let skipped = data.len() - wrote;

    if skipped > 0 {
        Err(anyhow!("skipped {} samples when writing", skipped))
    } else {
        Ok(())
    }
}

pub fn read_samples(consumer: &mut Consumer<f32>, size: usize) -> Result<Vec<f32>> {
    let mut samples = vec![0.0; size];
    let read = consumer.pop_slice(&mut samples);
    let missed = samples.len() - read;

    if missed > 0 {
        Err(anyhow!("missed {} samples when reading", missed))
    } else {
        Ok(samples)
    }
}
