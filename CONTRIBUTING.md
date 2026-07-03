# Contributing to Furia SDK Examples

## Getting Started
1. Fork the repo
2. Clone your fork
3. `cargo build --workspace`
4. `cargo test --workspace`

## Adding a new example
1. Copy an existing hello-* crate
2. Implement the SDK trait
3. Add to workspace members in Cargo.toml
4. Add tests

## Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Run `cargo clippy` before submitting
- Write tests for new functionality