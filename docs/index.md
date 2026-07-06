# Furia SDK Examples

Standalone example binaries for the Furia SDK traits — reference patterns for
C2 plugin developers.

## Quickstart

```bash
cargo run -p hello-sensor      # sensors
cargo run -p hello-policy      # governance
cargo run -p hello-simulation  # operations

# Run all tests
export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0
cargo test --release --workspace
```

## Layout

| Directory | Category |
|-----------|----------|
| `sensors/` | Acoustic, CBRN, EW, generic sensor |
| `entities/` | Battlespace objects, civilians, platforms |
| `intelligence/` | Assessment, fusion, decomposition, intent, terrain |
| `operations/` | Dispatch, logistics, simulation |
| `governance/` | Decision trees, policy/ROE |
| `integrations/` | Export, NATO coalition, UI |
| `cuas/` | Airspace, engagement, kill chain, threat scoring |

Browse the full [examples catalog](examples.md) for details on each crate.
