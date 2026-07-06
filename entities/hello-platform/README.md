# hello-platform

Demonstrates composing multiple SDK simulation providers into a single `ProviderRegistry` — a minimal platform runtime.

## What it shows

- Implementing `SimulationProvider` for drone and jammer modules
- Registering providers via a registry pattern
- Tick lifecycle and health aggregation across providers
- Health/version via `ModuleHandle`

## Run

```bash
cargo run -p hello-platform
```