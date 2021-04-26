use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Audio {
    pub input: Option<String>,
    pub output: Option<String>,
}
