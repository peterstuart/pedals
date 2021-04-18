use pedals::{audio, pedal::Delay, Pedal, Pipeline, Result};

fn main() -> Result<()> {
    let (input_device, output_device) = audio::devices()?;
    let config = audio::config(&input_device)?;

    audio::run(
        &input_device,
        &output_device,
        &config,
        Pipeline::new(vec![Delay::new(&config, 500.0, 0.25)?.boxed()])?,
    )
}
