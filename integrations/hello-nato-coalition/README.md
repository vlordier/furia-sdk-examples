# hello-nato-coalition

Demonstrates coalition-labelled module lifecycle, security context, health, logging, and audit using the types available in `furia-sdk` v0.1.0.

`Nation`, `MarkingProfile`, and `NationalCaveat` live in newer `furia-core` shared types, but they are not available from the published `v0.1.0` git tag consumed by this examples repository. This example therefore stays honest to the current dependency and avoids runtime TODO output.

## What it shows

- `SecurityContext` with a coalition exercise identity
- `ModuleHandle` logging, health reporting, and audit calls
- Local display of releasability/caveat labels as strings until the next SDK tag exposes shared NATO types

## Run

```bash
export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0
cargo run --release -p hello-nato-coalition
```
