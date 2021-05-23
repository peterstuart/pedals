use crate::{
    audio, config,
    effect::{Effect, Pipeline},
    util, Config, Result,
};
use audio::midi;
use std::{env, fs};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    util::set_panic_hook();

    main().unwrap();
}

fn main() -> Result<()> {
    let config = config()?;

    let input_device = config
        .audio
        .as_ref()
        .and_then(|audio| (&audio.input).clone());
    let output_device = config.audio.as_ref().and_then(|audio| audio.output.clone());

    let (input_device, output_device) = audio::devices(&input_device, &output_device)?;
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
}

fn config() -> Result<Config> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);

    match path {
        Some(path) => config_from_path(path),
        None => Ok(Config::default()),
    }
}

fn config_from_path(path: &str) -> Result<Config> {
    println!("Reading config from {}", path);

    let contents = fs::read_to_string(path)?;
    Config::from(&contents)
}
