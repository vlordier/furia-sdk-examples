# Furia SDK Examples

Standalone example binaries for the Furia SDK traits — reference patterns for C2 plugin developers.

## Quickstart

```bash
# Run a specific example
cargo run -p hello-decomposition
cargo run -p hello-policy
cargo run -p hello-simulation

# Run all tests
export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0
cargo test --release --workspace
```

## Examples

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-acoustic` | AcousticProvider | Acoustic threat detection |
| `hello-assessment` | AssessmentEngine | Threat/risk assessment |
| `hello-cbrn` | CbrnProvider | CBRN detection |
| `hello-civilian` | CivilianDensityProvider | Civilian behavior modeling |
| `hello-decision-tree` | DecisionTreeProvider | COA decision trees |
| `hello-decomposition` | DecompositionStrategy | Mission decomposition |
| `hello-dispatch` | DispatchAdapter | Asset dispatch |
| `hello-ew` | ElectronicWarfare | EW simulation |
| `hello-export` | ExportAdapter | Data export |
| `hello-fusion` | FusionEngine | Sensor fusion |
| `hello-intent` | IntentParser | Natural language intent |
| `hello-logistics` | LogisticsProvider | Logistics planning |
| `hello-nato-coalition` | ModuleHandle / SecurityContext | Coalition-labelled module lifecycle and audit demo (NATO domain types are planned for a later SDK tag) |
| `hello-platform` | PlatformProvider | Platform management |
| `hello-policy` | PolicyProvider | IHL/ROE policy |
| `hello-sensor` | SensorAdapter | Generic sensor |
| `hello-simulation` | SimulationProvider | Entity simulation |
| `hello-terrain` | TerrainProvider | Terrain analysis |
| `hello-ui` | (UI components) | SolidJS UI plugins |

## Architecture

Each example is a standalone binary that:
1. Implements or exercises one SDK trait (or related shared type family) with a local demo struct
2. Calls the trait methods directly in a minimal CLI program and unit tests
3. Uses `ModuleHandle` where the trait requires SDK context
4. Avoids platform registration so examples stay small and runnable without a host process

See also: [furia-plugin-example](https://github.com/vlordier/furia-plugin-example) (complete plugin), [furia-core](https://github.com/vlordier/furia-core) (SDK traits).


## Roadmap

The current workspace covers the core examples listed above. C-UAS trait examples are planned for `AirspaceManager`, `EngagementPlanner`, `InterceptorPairingProvider`, `KillChainOrchestrator`, and `ThreatScorer`.
