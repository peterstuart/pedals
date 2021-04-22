use crate::{AudioUnit, Result};

#[derive(Debug)]
pub struct Transparent {}

impl Transparent {
    pub fn new() -> Self {
        Self {}
    }
}

impl AudioUnit for Transparent {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        output.copy_from_slice(input);
        Ok(())
    }
}

impl Default for Transparent {
    fn default() -> Self {
        Self::new()
    }
}
