# hello-sensor

Demonstrates the `SensorAdapter` SDK trait — ingesting and classifying radar track data.

## What it shows

- Implementing `SensorAdapter` for CSV payload parsing
- Classifying tracks by affiliation (hostile, friendly, unknown)
- Health/version via `ModuleHandle`

## Run

```bash
cargo run -p hello-sensor
```