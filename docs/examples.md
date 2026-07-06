# Examples Catalog

All 26 example crates organized by category.


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

## See Also

- [Architecture](architecture.md)

