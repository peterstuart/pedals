use crate::Result;
use anyhow::anyhow;
use ringbuf::{Consumer, Producer};

pub fn write_empty_samples(producer: &mut Producer<f32>, size: usize) -> Result<()> {
    for _ in 0..size {
        producer
            .push(0.0)
            .map_err(|_| anyhow!("failed to write empty samples"))?;
    }

    Ok(())
}

pub fn write_frame(producer: &mut Producer<f32>, data: &[f32]) -> Result<()> {
    for &sample in data {
        producer
            .push(sample)
            .map_err(|_| anyhow!("failed to write frame"))?;
    }

    Ok(())
}

pub fn read_frame(consumer: &mut Consumer<f32>, size: usize) -> Result<Vec<f32>> {
    let mut frame = vec![0.0; size];

    for sample in frame.iter_mut() {
        *sample = consumer
            .pop()
            .ok_or_else(|| anyhow!("failed to read frame"))?;
    }

    Ok(frame)
}
