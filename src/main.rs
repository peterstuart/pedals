use audio::midi;
use pedals::{
    audio,
    effect::{Effect, Pipeline},
    Config, Result,
};
use std::{env, fs};

fn main() -> Result<()> {
    let config = config()?;

    let (input_device, output_device) = audio::devices(&config.audio)?;
    let stream_config = audio::config(&input_device)?;
    let pipeline = Pipeline::from(&config, &stream_config)?.boxed();

    let midi_port_names = midi::port_names()?;

    if config.midi.port.is_none() && !midi_port_names.is_empty() {
        println!(
            "Config is missing 'midi.port'. Available MIDI ports are:\n{}",
            midi_port_names.join("\n")
        );
    }

    audio::run(
        config.audio.latency_ms,
        &input_device,
        &output_device,
        &stream_config,
        &config.midi,
        pipeline,
    )
}

fn config() -> Result<Config> {
    let args: Vec<String> = env::args().collect();
    let config = args.get(1).map(|path| config_from_path(path)).transpose()?;
    Ok(config.unwrap_or_default())
}

fn config_from_path(path: &str) -> Result<Config> {
    println!("Reading config from {}", path);

    let contents = fs::read_to_string(path)?;
    Config::from(&contents)
}
