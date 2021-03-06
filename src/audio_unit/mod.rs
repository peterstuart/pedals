pub mod delay;
pub mod looper;

mod fft;
mod gain;
mod pipeline;
mod split;
mod transparent;

pub use delay::Delay;
pub use fft::Fft;
pub use gain::Gain;
pub use looper::Looper;
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
