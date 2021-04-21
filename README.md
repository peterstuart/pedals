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
pedals:
  - type: Delay
    delay_ms: 250
    level: 0.5
    num: 6
```

```shell
cargo run -- pipeline.yml
```

