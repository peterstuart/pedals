use crate::{
    audio, config,
    effect::{Effect, Pipeline},
    util, Config,
};
use audio::midi;
use std::{env, fs};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

#[wasm_bindgen]
pub struct Handle(Stream);

#[wasm_bindgen]
pub fn beep() -> Handle {
    console::log_1(&"beep 1".into());

    let host = cpal::default_host();

    console::log_1(&"beep 2".into());

    if let Err(e) = host.default_output_device() {
        console::log_2(&"error".into(), &e.into());
    }

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    console::log_1(&"beep 3".into());

    let config = device.default_output_config().unwrap();

    console::log_1(&"beep 4".into());

    Handle(match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    })
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Stream
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
    };

    let err_fn = |err| console::error_1(&format!("an error occurred on stream: {}", err).into());

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _| write_data(data, channels, &mut next_value),
            err_fn,
        )
        .unwrap();
    stream.play().unwrap();
    stream
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

/*

#[wasm_bindgen]
pub fn run() {
    util::set_panic_hook();

    match main() {
        Ok(_) => alert("finished"),
        Err(e) => alert(&e.to_string()),
    }
}

fn main() -> Result<()> {
    let config = Config::default();

    let input_device = config
        .audio
        .as_ref()
        .and_then(|audio| (&audio.input).clone());
    let output_device = config.audio.as_ref().and_then(|audio| audio.output.clone());

    let (input_device, output_device) = audio::devices(&input_device, &output_device)?;

    /*
    let stream_config = audio::config(&input_device)?;
    let pipeline = Pipeline::from(&config, &stream_config)?.boxed();

         let midi_port_names = midi::port_names()?;

         if config.midi.is_none() && !midi_port_names.is_empty() {
             println!(
                 "Config is missing 'midi'. Available MIDI ports are:\n{}",
                 midi_port_names.join("\n")
             );
         }

         audio::run(
             &input_device,
             &output_device,
             &stream_config,
             &config.midi.unwrap_or_else(config::Midi::default),
             pipeline,
         )
    */

    Ok(())
}

*/
