use pedals::{audio, Pedal, Pipeline, Result, Transparent};

fn main() -> Result<()> {
    let pipeline = Pipeline::new(vec![Transparent::new().boxed()])?;
    audio::run(pipeline)
}
