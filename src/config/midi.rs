use crate::Result;
use serde::Deserialize;
use wmidi::Channel;

#[derive(Debug, Deserialize)]
pub struct Midi {
    pub port: String,
    channel: u8,
}

impl Midi {
    pub fn channel(&self) -> Result<Channel> {
        Ok(Channel::from_index(self.channel - 1)?)
    }
}
