use crate::{audio_unit::AudioUnit, util, Result};
use cpal::StreamConfig;
use std::sync::mpsc::{self, Receiver, Sender};
use State::*;

#[derive(Debug)]
pub enum Message {
    Toggle,
    QueueOverdub,
    TickMeasure,
}

#[derive(Debug)]
enum State {
    Off,
    QueueRecording,
    Recording { position: usize },
    QueuePlaying { position: usize },
    Playing { position: usize, total: usize },
    PlayingAwaitingOverdub { position: usize, total: usize },
    Overdubbing { position: usize, total: usize },
}

impl State {
    pub fn is_awaiting_overdub(&self) -> bool {
        matches!(self, State::PlayingAwaitingOverdub { .. })
    }

    pub fn start_overdubbing(&mut self, position: usize) {
        if let State::PlayingAwaitingOverdub { total, .. } = self {
            println!("looper: activating overdub");
            *self = Overdubbing {
                position,
                total: *total,
            };
        } else {
            panic!("Called start_overdubbing() while not awaiting overdub");
        }
    }

    pub fn update_position(&mut self, position: usize) {
        match self {
            State::Playing { total, .. } => {
                *self = Playing {
                    position,
                    total: *total,
                };
            }
            State::PlayingAwaitingOverdub { total, .. } => {
                *self = PlayingAwaitingOverdub {
                    position,
                    total: *total,
                };
            }
            _ => {
                panic!("Called update_position() when not playing: {:?}", self);
            }
        }
    }

    pub fn queue_overdub(&mut self) {
        if let Playing { position, total } = self {
            println!("looper: enabling overdub mode");
            *self = PlayingAwaitingOverdub {
                position: *position,
                total: *total,
            };
        }
    }

    pub fn queue_toggle(&mut self) {
        *self = match self {
            Off => QueueRecording,
            Recording { position } => QueuePlaying {
                position: *position,
            },
            _ => Off,
        };

        println!("looper: {:?}", self);
    }

    pub fn tick_measure(&mut self) {
        if let QueueRecording = self {
            println!("looper: Recording");
            *self = Recording { position: 0 }
        } else if let QueuePlaying { position } = self {
            println!("looper: Playing");
            *self = Playing {
                position: 0,
                total: *position,
            }
        }
    }
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
                self.state.queue_toggle();
            }
            Message::QueueOverdub => {
                self.state.queue_overdub();
            }
            Message::TickMeasure => {
                self.state.tick_measure();
            }
        }
    }

    fn process_samples(&mut self, input: &[f32], output: &mut [f32]) {
        match self.state {
            Off | QueueRecording => {
                util::zero_slice(output);
            }
            Recording { position } | QueuePlaying { position } => {
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
                        self.state = if matches!(self.state, Recording { .. }) {
                            Recording {
                                position: next_position,
                            }
                        } else {
                            QueuePlaying {
                                position: next_position,
                            }
                        };
                    }
                } else {
                    self.process_samples_wrap_around(position, self.buffer.len(), input, output);
                }
            }
            Playing { position, total } | PlayingAwaitingOverdub { position, total } => {
                self.process_samples_playing(position, total, input, output);
            }
            Overdubbing { position, total } => {
                let next_position = position + output.len();

                if next_position <= total {
                    output.copy_from_slice(&self.buffer[position..next_position]);

                    for (sample_number, input_sample) in input.iter().enumerate() {
                        self.buffer[position + sample_number] += input_sample;
                    }

                    let position = if next_position == total {
                        0
                    } else {
                        next_position
                    };

                    self.state = if next_position == total {
                        println!("looper: done overdubbing");
                        Playing { position, total }
                    } else {
                        Overdubbing { position, total }
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

            if next_position == total && self.state.is_awaiting_overdub() {
                self.state.start_overdubbing(position);
            } else {
                self.state.update_position(position);
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
