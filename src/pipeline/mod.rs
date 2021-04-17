use std::fmt::Display;

use crate::{pedal, Result};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Pipeline {
    pedals: Vec<pedal::Boxed>,
}

impl Pipeline {
    pub fn new(pedals: Vec<pedal::Boxed>) -> Result<Self> {
        if pedals.is_empty() {
            Err(anyhow!("Must have at least one pedal in the pipeline"))
        } else {
            Ok(Self { pedals })
        }
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let mut input = input.to_vec();

        for pedal in &mut self.pedals {
            pedal.process(&input, output);
            input.copy_from_slice(output);
        }
    }
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut names: Vec<_> = self.pedals.iter().map(|pedal| pedal.name()).collect();
        names.insert(0, "Input".into());
        names.insert(names.len(), "Output".into());

        names.join(" -> ").fmt(f)
    }
}
