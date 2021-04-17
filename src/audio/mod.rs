use crate::{Pipeline, Result};
use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::time::Duration;

const LATENCY: f32 = 20.0;

pub fn run(mut pipeline: Pipeline) -> Result<()> {
    let (input_device, output_device) = devices()?;

    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    let latency_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    for _ in 0..latency_samples {
        producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let output_fell_behind = write_frame(&mut producer, data);

        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let (frame, input_fell_behind) = read_frame(&mut consumer, output.len());
        pipeline.process(&frame, output);

        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    println!(
        "Attempting to build both streams with f32 samples and {:?}.",
        config
    );
    let input_stream = input_device.build_input_stream(&config, input_data_fn, handle_error)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, handle_error)?;
    println!("Successfully built streams.");

    println!(
        "Starting the input and output streams with {} milliseconds of latency.",
        LATENCY
    );
    input_stream.play()?;
    output_stream.play()?;

    std::thread::sleep(Duration::from_secs(u64::MAX));

    drop(input_stream);
    drop(output_stream);

    Ok(())
}

fn devices() -> Result<(Device, Device)> {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .ok_or(anyhow!("Failed to find input device"))?;
    println!("Input device: {}", input_device.name()?);

    let output_device = host
        .default_output_device()
        .ok_or(anyhow!("Failed to find output device"))?;
    println!("Output device {}", output_device.name()?);

    Ok((input_device, output_device))
}

fn write_frame(producer: &mut Producer<f32>, data: &[f32]) -> bool {
    let mut fell_behind = false;

    for &sample in data {
        if producer.push(sample).is_err() {
            fell_behind = true;
        }
    }

    fell_behind
}

fn read_frame(consumer: &mut Consumer<f32>, size: usize) -> (Vec<f32>, bool) {
    let mut frame = vec![0.0; size];
    let mut fell_behind = false;

    for i in 0..size {
        frame[i] = match consumer.pop() {
            Some(s) => s,
            None => {
                fell_behind = true;
                0.0
            }
        };
    }

    (frame, fell_behind)
}

fn handle_error(error: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", error);
}
