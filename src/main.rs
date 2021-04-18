use std::{env, fs};

use pedals::{audio, pedal::Config, Pipeline, Result};

fn main() -> Result<()> {
    let config = config()?;
    let (input_device, output_device) = audio::devices()?;
    let stream_config = audio::config(&input_device)?;
    let pipeline = Pipeline::from(&config, &stream_config)?;

    audio::run(&input_device, &output_device, &stream_config, pipeline)
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
    Ok(serde_yaml::from_str(&contents)?)
}
