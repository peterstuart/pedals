pub mod delay;

mod gain;
mod pipeline;
mod split;
mod transparent;

pub use delay::Delay;
pub use gain::Gain;
pub use pipeline::Pipeline;
pub use split::Split;
pub use transparent::Transparent;

use crate::Result;
use wmidi::MidiMessage;

pub type Boxed = Box<dyn AudioUnit>;

pub trait AudioUnit: Send + Sync {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()>;

    fn handle_midi_messages(&mut self, _midi_messages: &[MidiMessage<'static>]) -> Result<()> {
        Ok(())
    }

    fn boxed(self) -> Boxed
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
