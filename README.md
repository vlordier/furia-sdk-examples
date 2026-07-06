# Furia SDK Examples

Standalone example binaries for the Furia SDK traits — reference patterns for C2 plugin developers.

## Quickstart

```bash
# Pick a category and run any example:
cargo run -p hello-sensor      # sensors
cargo run -p hello-policy      # governance
cargo run -p hello-simulation  # operations

# Run all tests
export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0
cargo test --release --workspace
```

## Examples

### Sensors

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-acoustic` | AcousticProvider | Acoustic threat detection |
| `hello-cbrn` | CbrnProvider | CBRN detection |
| `hello-ew` | ElectronicWarfare | EW simulation |
| `hello-sensor` | SensorAdapter | Generic sensor |

### Entities

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-bso` | BattlespaceObject | COP battlespace object |
| `hello-civilian` | CivilianDensityProvider | Civilian behavior modeling |
| `hello-platform` | PlatformProvider | Platform management |

### Intelligence

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-assessment` | AssessmentEngine | Threat/risk assessment |
| `hello-decomposition` | DecompositionStrategy | Mission decomposition |
| `hello-fusion` | FusionEngine | Sensor fusion |
| `hello-intent` | IntentParser | Natural language intent |
| `hello-terrain` | TerrainProvider | Terrain analysis |

### Operations

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-dispatch` | DispatchAdapter | Asset dispatch |
| `hello-logistics` | LogisticsProvider | Logistics planning |
| `hello-simulation` | SimulationProvider | Entity simulation |

### Governance

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-decision-tree` | DecisionTreeProvider | COA decision trees |
| `hello-policy` | PolicyProvider | IHL/ROE policy |

### Integrations

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-export` | ExportAdapter | Data export |
| `hello-nato-coalition` | ModuleHandle / SecurityContext | Coalition-labelled module lifecycle and audit demo (NATO domain types are planned for a later SDK tag) |
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
