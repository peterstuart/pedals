use anyhow::anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use pedals::Result;
use ringbuf::RingBuffer;

const LATENCY: f32 = 20.0;

fn main() -> Result<()> {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .ok_or(anyhow!("Failed to find input device"))?;
    println!("Input device: {}", input_device.name()?);

    let output_device = host
        .default_output_device()
        .ok_or(anyhow!("Failed to find output device"))?;
    println!("Output device {}", output_device.name()?);

    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (LATENCY / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    println!(
        "Attempting to build both streams with f32 samples and `{:?}`.",
        config
    );
    let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn)?;
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn)?;
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with {} milliseconds of latency.",
        LATENCY
    );
    input_stream.play()?;
    output_stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(input_stream);
    drop(output_stream);

    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
