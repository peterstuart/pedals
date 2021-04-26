## Setup

Install [`rustup`](https://rustup.rs/). Install the [`rust-analyzer`](https://rust-analyzer.github.io) language server if desired.

## Formatting

```shell
cargo fmt
```

## Linting

```shell
cargo clippy
```

## Tests

```shell
cargo test
```

## Running

Create a configuration file for the pipeline:

```yaml
# omit "audio" to use the defaults
# all fields in "audio" are optional
audio:
  input: # put any string here to have pedals show you a list of available devices
  output: # ditto
# midi is optional
midi:
  port: # put any string here to have pedals show you a list of available ports
effects:
  - type: Delay
    delay_ms: 250
    level: 0.5
    num: 6
	# midi_slider is optional
    midi_slider:
      channel: 1
      control_change: 1 # modulation wheel
```

```shell
cargo run -- pipeline.yml
```

