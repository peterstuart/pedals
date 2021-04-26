pub mod midi;

use crate::{ring_buffer, AudioUnit, Pipeline, Result};
use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};
use ringbuf::RingBuffer;
use std::time::Duration;
use wmidi::MidiMessage;

const LATENCY: f32 = 50.0;

pub fn run(
    input_device: &Device,
    output_device: &Device,
    config: &StreamConfig,
    midi_port_name: Option<String>,
    mut pipeline: Pipeline,
) -> Result<()> {
    let latency_num_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
    let latency_num_samples = latency_num_frames as usize * config.channels as usize;

    let ring = RingBuffer::new(latency_num_samples * 2);
    let (mut producer, mut consumer) = ring.split();
    ring_buffer::write_empty_samples(&mut producer, latency_num_samples)?;

    let midi_messages = match midi_port_name {
        Some(midi_port_name) => Some(midi::listen_for_input(&midi_port_name)?),
        None => None,
    };

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        if let Err(e) = ring_buffer::write_frame(&mut producer, data) {
            eprintln!("input: {:?}", e);
        }
    };

    let output_data_fn = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        if let Some(midi_messages) = &midi_messages {
            let midi_messages: Vec<MidiMessage<'static>> = midi_messages.try_iter().collect();
            if let Err(e) = pipeline.handle_midi_messages(&midi_messages) {
                eprintln!("error handling midi messages: {:?}", e);
            }
        }

        if let Err(e) = ring_buffer::read_samples(&mut consumer, output.len())
            .and_then(|frame| pipeline.process(&frame, output))
        {
            eprintln!("output: {:?}", e);
        };
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

pub fn devices() -> Result<(Device, Device)> {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .ok_or_else(|| anyhow!("Failed to find input device"))?;
    println!("Input device: {}", input_device.name()?);

    let output_device = host
        .default_output_device()
        .ok_or_else(|| anyhow!("Failed to find output device"))?;
    println!("Output device {}", output_device.name()?);

    Ok((input_device, output_device))
}

pub fn config(device: &Device) -> Result<StreamConfig> {
    Ok(device.default_input_config()?.into())
}

fn handle_error(error: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", error);
}
