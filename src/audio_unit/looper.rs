use crate::{audio_unit::AudioUnit, util, Result};
use cpal::StreamConfig;
use std::sync::mpsc::{self, Receiver, Sender};
use State::*;

#[derive(Debug)]
pub enum Message {
    Toggle,
    EnableOverdubMode,
}

#[derive(Debug)]
enum State {
    Off,
    Recording { position: usize },
    Playing { position: usize, total: usize },
    PlayingAwaitingOverdub { position: usize, total: usize },
    Overdubbing { position: usize, total: usize },
}

pub struct Looper {
    messages: Receiver<Message>,
    buffer: Vec<f32>,
    state: State,
}

impl Looper {
    pub fn new(stream_config: &StreamConfig, max_buffer_ms: u32) -> (Self, Sender<Message>) {
        let (sender, receiver) = mpsc::channel();

        let buffer = vec![0.0; util::ms_in_samples(stream_config, max_buffer_ms)];

        (
            Self {
                messages: receiver,
                buffer,
                state: Off,
            },
            sender,
        )
    }

    fn process_messages(&mut self) {
        let messages: Vec<_> = self.messages.try_iter().collect();
        for message in messages {
            self.process_message(message);
        }
    }

    fn process_message(&mut self, message: Message) {
        match message {
            Message::Toggle => {
                self.state = match self.state {
                    Off => Recording { position: 0 },
                    Recording { position } => Playing {
                        position: 0,
                        total: position,
                    },
                    Playing {
                        position: _,
                        total: _,
                    } => Off,
                    PlayingAwaitingOverdub {
                        position: _,
                        total: _,
                    } => Off,
                    Overdubbing {
                        position: _,
                        total: _,
                    } => Off,
                };

                println!("looper: {:?}", self.state);
            }
            Message::EnableOverdubMode => {
                if let Playing { position, total } = self.state {
                    self.state = PlayingAwaitingOverdub { position, total };

                    println!("looper: enabling overdub mode");
                }
            }
        }
    }

    fn process_samples(&mut self, input: &[f32], output: &mut [f32]) {
        match self.state {
            Off => {
                util::zero_slice(output);
            }
            Recording { position } => {
                let next_position = position + input.len();

                if next_position <= self.buffer.len() {
                    self.buffer[position..next_position].copy_from_slice(input);

                    if next_position == self.buffer.len() {
                        println!("looper: out of space in the buffer. switching to playback");

                        self.state = Playing {
                            position: 0,
                            total: next_position,
                        };
                    } else {
                        self.state = Recording {
                            position: next_position,
                        };
                    }
                } else {
                    self.process_samples_wrap_around(position, self.buffer.len(), input, output);
                }
            }
            Playing { position, total } => {
                self.process_samples_playing(position, total, input, output);
            }
            PlayingAwaitingOverdub { position, total } => {
                self.process_samples_playing(position, total, input, output);
            }
            Overdubbing { position, total } => {
                let next_position = position + output.len();

                if next_position <= total {
                    output.copy_from_slice(&self.buffer[position..next_position]);
                    let position = if next_position == total {
                        0
                    } else {
                        next_position
                    };

                    self.state = match next_position == total {
                        true => {
                            println!("looper: done overdubbing");
                            Playing { position, total }
                        }
                        _ => Overdubbing { position, total },
                    }
                } else {
                    self.process_samples_wrap_around(position, total, input, output);
                }
            }
        };
    }

    fn process_samples_playing(
        &mut self,
        position: usize,
        total: usize,
        input: &[f32],
        output: &mut [f32],
    ) {
        let next_position = position + output.len();

        if next_position <= total {
            output.copy_from_slice(&self.buffer[position..next_position]);
            let position = if next_position == total {
                0
            } else {
                next_position
            };

            self.state = match self.state {
                PlayingAwaitingOverdub { .. } if next_position == total => {
                    println!("looper: activating overdub");
                    Overdubbing { position, total }
                }
                PlayingAwaitingOverdub { .. } => PlayingAwaitingOverdub { position, total },
                _ => Playing { position, total },
            }
        } else {
            self.process_samples_wrap_around(position, total, input, output);
        }
    }

    fn process_samples_wrap_around(
        &mut self,
        position: usize,
        total: usize,
        input: &[f32],
        output: &mut [f32],
    ) {
        let split_position = total - position;

        // play/record to end of buffer
        self.process_samples(&input[0..split_position], &mut output[0..split_position]);

        // ... then wrap around
        self.process_samples(
            &input[split_position..input.len()],
            &mut output[split_position..input.len()],
        )
    }
}

impl AudioUnit for Looper {
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<()> {
        self.process_messages();
        self.process_samples(input, output);

        Ok(())
    }
}
