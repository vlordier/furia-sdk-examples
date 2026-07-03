# Furia SDK Examples

Example implementations of every Furia SDK trait — reference patterns for C2 plugin developers.

## Quickstart

```bash
# List all examples
cargo build --workspace 2>&1 | grep "Compiling hello-" | head -20

# Run a specific example
cargo run -p hello-decomposition
cargo run -p hello-policy
cargo run -p hello-simulation

# Run all tests
cargo test --workspace
```

## Examples

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-acoustic` | AcousticSensor | Acoustic threat detection |
| `hello-assessment` | AssessmentProvider | Threat/risk assessment |
| `hello-cbrn` | CbrnSensor | CBRN detection |
| `hello-civilian` | CivilianModel | Civilian behavior modeling |
| `hello-decision-tree` | DecisionEngine | COA decision trees |
| `hello-decomposition` | DecompositionStrategy | Mission decomposition |
| `hello-dispatch` | DispatchAdapter | Asset dispatch |
| `hello-ew` | ElectronicWarfare | EW simulation |
| `hello-export` | ExportAdapter | Data export |
| `hello-fusion` | FusionEngine | Sensor fusion |
| `hello-intent` | IntentParser | Natural language intent |
| `hello-logistics` | LogisticsProvider | Logistics planning |
| `hello-nato-coalition` | (SDK pattern) | NATO coalition ops (SDK example) |
| `hello-platform` | PlatformProvider | Platform management |
| `hello-policy` | PolicyProvider | IHL/ROE policy |
| `hello-sensor` | SensorAdapter | Generic sensor |
| `hello-simulation` | SimulationProvider | Entity simulation |
| `hello-terrain` | TerrainProvider | Terrain analysis |
| `hello-ui` | (UI components) | SolidJS UI plugins |

## Architecture

Each example is a standalone binary that:
1. Implements one SDK trait
2. Registers via `FuriaBuilder::with_provider()`
3. Exposes health/version via `ModuleHandle`
4. Demonstrates the provider in a minimal service context

See also: [furia-plugin-example](https://github.com/vlordier/furia-plugin-example) (complete plugin), [furia-core](https://github.com/vlordier/furia-core) (SDK traits).
