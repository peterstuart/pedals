use crate::{pedal, util, Pedal, Result};
use anyhow::anyhow;

pub struct Split {
    pedals: Vec<pedal::Boxed>,
}

impl Split {
    pub fn new(pedals: Vec<pedal::Boxed>) -> Result<Self> {
        if pedals.is_empty() {
            Err(anyhow!("Must have at least one pedal in the Split"))
        } else {
            Ok(Self { pedals })
        }
    }
}

impl Pedal for Split {
    fn name(&self) -> String {
        "Split".into()
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        let length = output.len();
        util::zero_slice(output);

        let mut pedal_output: Vec<f32> = vec![0.0; length];

        for pedal in &mut self.pedals {
            pedal.process(input, &mut pedal_output)?;

            for i in 0..output.len() {
                output[i] += pedal_output[i];
            }
        }

        Ok(())
    }
}
