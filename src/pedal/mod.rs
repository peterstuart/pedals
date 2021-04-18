mod delay;
mod transparent;

pub use delay::Delay;
pub use transparent::Transparent;

use crate::Result;
use std::fmt::Display;

pub type Boxed = Box<dyn Pedal>;

pub trait Pedal: Send + Sync {
    fn name(&self) -> String;

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()>;

    fn boxed(self) -> Boxed
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}

impl Display for dyn Pedal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.name(), f)
    }
}
