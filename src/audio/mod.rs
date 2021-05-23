pub mod midi;

use crate::{config, effect, ring_buffer, Result};
use anyhow::anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, StreamConfig,
};
use ringbuf::RingBuffer;
use std::time::Duration;

const LATENCY: f32 = 50.0;

pub fn run(
    input_device: &Device,
    output_device: &Device,
    config: &StreamConfig,
    midi_config: &config::Midi,
    mut effect: effect::Boxed,
) -> Result<()> {
    let latency_num_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
    let latency_num_samples = latency_num_frames as usize * config.channels as usize;

    let ring = RingBuffer::new(latency_num_samples * 2);
    let (mut producer, mut consumer) = ring.split();
    ring_buffer::write_empty_samples(&mut producer, latency_num_samples)?;

    let (midi_messages, _midi_input) = match &midi_config.port {
        Some(port_name) => {
            let (midi_messages, midi_input) = midi::listen_for_input(port_name)?;
            (Some(midi_messages), Some(midi_input))
        }
        None => (None, None),
    };

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        if let Err(e) = ring_buffer::write_samples(&mut producer, data) {
            eprintln!("input: {:?}", e);
        }
    };

    let output_data_fn = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let midi_messages = midi_messages
            .as_ref()
            .map_or(vec![], |midi_messages| midi_messages.try_iter().collect());

        if let Err(e) = ring_buffer::read_samples(&mut consumer, output.len())
            .and_then(|frame| effect.process(&midi_messages, &frame, output))
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

pub fn devices(
    input_device: &Option<String>,
    output_device: &Option<String>,
) -> Result<(Device, Device)> {
    let host = cpal::default_host();

    let input_device = match input_device {
        Some(name) => device(&host, &name),
        None => host
            .default_input_device()
            .ok_or_else(|| anyhow!("Failed to find input device")),
    }?;

    println!("Input device: {}", input_device.name()?);

    let output_device = match output_device {
        Some(name) => device(&host, &name),
        None => host
            .default_output_device()
            .ok_or_else(|| anyhow!("Failed to find output device")),
    }?;

    println!("Output device {}", output_device.name()?);

    Ok((input_device, output_device))
}

pub fn device(host: &Host, name: &str) -> Result<Device> {
    let device_names = host
        .devices()?
        .map(|device| {
            device
                .name()
                .map_err(|e| anyhow!("Could not get device name: {}", e))
        })
        .collect::<Result<Vec<String>>>()?;

    host.devices()?
        .find(|device| device.name().map_or(false, |n| n == name))
        .ok_or_else(|| {
            anyhow!(
                "Could not find an audio device with name '{}'. Available devices are:\n{}",
                name,
                device_names.join("\n")
            )
        })
}

pub fn config(device: &Device) -> Result<StreamConfig> {
    Ok(device.default_input_config()?.into())
}

fn handle_error(error: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", error);
}
