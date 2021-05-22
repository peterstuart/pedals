mod delay;
mod pipeline;
mod tap_tempo;
mod tempo;
mod transparent;

pub use delay::Delay;
pub use pipeline::Pipeline;
pub use tempo::Tempo;
pub use transparent::Transparent;

use crate::{audio::midi::Message, config, Result};
use cpal::StreamConfig;

pub type Boxed = Box<dyn Effect>;

pub trait Effect: Send {
    fn process(
        &mut self,
        midi_messages: &[Message],
        input: &[f32],
        output: &mut [f32],
    ) -> Result<()>;

    fn boxed(self) -> Boxed
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}

pub fn from(config: config::Effect, stream_config: &StreamConfig) -> Result<Boxed> {
    Ok(match config {
        config::Effect::Transparent => Transparent::new().boxed(),
        config::Effect::Delay(delay_config) => Delay::new(delay_config, stream_config)?.boxed(),
    })
}
