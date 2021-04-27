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

pub type Boxed = Box<dyn AudioUnit>;

pub trait AudioUnit: Send {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()>;

    fn boxed(self) -> Boxed
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
