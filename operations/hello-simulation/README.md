# hello-simulation

Demonstrates the `SimulationProvider` SDK trait — the core simulation runtime for entity behaviour.

## What it shows

- Implementing `SimulationProvider` for a patrol drone simulator
- Initialising from a `Scenario`, ticking over time, exposing entity state
- Health reporting (Healthy, Degraded, Unhealthy)
- Health/version via `ModuleHandle`

## Run

```bash
cargo run -p hello-simulation
```