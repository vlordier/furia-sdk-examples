# Contributing to Furia SDK Examples

## Getting Started
1. Fork the repo
2. Clone your fork
3. `cargo build --workspace`
4. `cargo test --workspace`

## Adding a New Example (`hello-*` crate)

1. Copy an existing crate (e.g. `sensors/hello-sensor`) as a template and place it in the matching category directory
2. Rename the crate and implement the SDK trait from `furia-sdk`
3. Register it in the workspace `Cargo.toml` if needed (per-category globs auto-discover — add to the appropriate category in `members`)
4. Add `[lints] workspace = true` to the new crate's `Cargo.toml`
5. Add a row to the examples table: `./scripts/generate-docs.sh` (auto-generates `README.md` and `docs/examples.md`)
6. Write tests demonstrating the trait's key methods
7. Run `cargo check --workspace` and `cargo test --workspace` before submitting

## What Makes a Good Example

- **Self-contained**: A single `main.rs` that demonstrates the trait in ~50-100 lines
- **Faithful to the trait**: Show the intended usage pattern, not a workaround
- **Minimal dependencies**: Only depend on `furia-sdk`, `serde`, `serde_json` unless you have a strong reason
- **Runnable**: `cargo run -p hello-<name>` should produce meaningful output
- **Documented**: Top-level doc comment explaining what the example demonstrates

## Test Conventions

- Tests live in the same file as the implementation (inline `#[cfg(test)]` modules)
- Each test covers one method or one scenario — prefer 3-5 focused tests over one giant test
- Use `#[test]` directly (no tokio::test needed unless the trait is async)
- Run `cargo test --workspace` before pushing

## PR Checklist

- [ ] `cargo check --workspace` passes with no warnings
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all` has been run
- [ ] README.md example table is regenerated: `./scripts/generate-docs.sh`
- [ ] `[lints] workspace = true` is present in the new crate's Cargo.toml

## Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Run `cargo clippy` before submitting
- Write tests for new functionality
- Use `cargo check` for fast iteration (avoid full builds when possible)