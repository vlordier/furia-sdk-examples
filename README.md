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
| `hello-acoustic` | AcousticProvider | passive sonar detection |
| `hello-cbrn` | CbrnProvider | Gaussian plume dispersion |
| `hello-ew` | EWSimProvider | jammer vs radio link |
| `hello-sensor` | SensorAdapter | ingest radar track data |

### Entities

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-bso` | BattlespaceObject | the authoritative COP track type (STANAG 4255 / APP-6) |
| `hello-civilian` | CivilianDensityProvider | population by region |
| `hello-platform` |  | composing multiple providers into a platform using ProviderRegistry |

### Intelligence

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-assessment` | AssessmentEngine | battle damage assessment |
| `hello-decomposition` | DecompositionStrategy | mission into sub-phases |
| `hello-fusion` | FusionEngine | correlate sensor tracks |
| `hello-intent` | IntentProvider | parse commander intent text |
| `hello-terrain` | TerrainAnalyst | slope-based mobility classification |

### Operations

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-dispatch` | DispatchAdapter | send action to a target |
| `hello-logistics` | LogisticsProvider | convoy fuel tracking |
| `hello-simulation` | SimulationProvider | drone patrol with fuel consumption |

### Governance

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-decision-tree` | DecisionTreeProvider | threshold classifier |
| `hello-policy` | PolicyProvider | ROE with civilian protection |

### Integrations

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-export` | ExportAdapter | export entities to JSON |
| `hello-nato-coalition` |  | using ModuleHandle and SecurityContext — NATO coalition lifecycle and audit |
| `hello-ui` | UiPlugin | custom panel metadata |

### C-UAS / Air Defence

| Crate | SDK Trait | Description |
|-------|-----------|-------------|
| `hello-airspace` | AirspaceManager | no-fly zones and deconfliction |
| `hello-engagement` | EngagementPlanner | salvo computation and launch timing |
| `hello-interceptor-pairing` | InterceptorPairingProvider | map threats to effectors |
| `hello-kill-chain` | KillChainOrchestrator | D-DIL automation for C-UAS |
| `hello-threat-scorer` | ThreatScorer | classify drones and assign threat levels |

## Architecture

Each example is a standalone binary that:
1. Implements or exercises one SDK trait (or related shared type family) with a local demo struct
2. Calls the trait methods directly in a minimal CLI program and unit tests
3. Uses `ModuleHandle` where the trait requires SDK context
4. Avoids platform registration so examples stay small and runnable without a host process

See also: [furia-plugin-example](https://github.com/vlordier/furia-plugin-example) (complete plugin), [furia-core](https://github.com/vlordier/furia-core) (SDK traits).

