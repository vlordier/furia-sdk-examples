# Hierarchical Examples Structure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restructure flat `hello-*` crate directories into a hierarchical category tree without changing package names or breaking existing workflows.

**Architecture:** Six category directories (`sensors/`, `entities/`, `intelligence/`, `operations/`, `governance/`, `integrations/`) at workspace root, each containing their respective `hello-*` crates. Workspace `Cargo.toml` members glob updated from `"hello-*"` to per-category globs. Package names and `cargo run -p hello-*` commands unchanged.

**Tech Stack:** Rust, Cargo workspace, git

## Global Constraints

- No source code changes inside any crate
- All `cargo run -p hello-*` commands must continue working
- `cargo test --workspace` must pass

---

### Task 1: Move crates into category directories

**Files:**
- Create: `sensors/`, `entities/`, `intelligence/`, `operations/`, `governance/`, `integrations/`
- Move: each `hello-*` dir into its category dir via `git mv`

- [ ] **Step 1: Create category dirs and git mv crates**

```bash
mkdir -p sensors entities intelligence operations governance integrations

# sensors
git mv hello-acoustic sensors/
git mv hello-cbrn sensors/
git mv hello-ew sensors/
git mv hello-sensor sensors/

# entities
git mv hello-bso entities/
git mv hello-civilian entities/
git mv hello-platform entities/

# intelligence
git mv hello-assessment intelligence/
git mv hello-decomposition intelligence/
git mv hello-fusion intelligence/
git mv hello-intent intelligence/
git mv hello-terrain intelligence/

# operations
git mv hello-dispatch operations/
git mv hello-logistics operations/
git mv hello-simulation operations/

# governance
git mv hello-decision-tree governance/
git mv hello-policy governance/

# integrations
git mv hello-export integrations/
git mv hello-nato-coalition integrations/
git mv hello-ui integrations/
```

- [ ] **Step 2: Verify git status**

```bash
git status --short
```
Expected: 20 renamed entries (R100 from `hello-*` to `category/hello-*`)

- [ ] **Step 3: Commit**

```bash
git add -A && git commit -m "refactor: group hello-* crates into category directories

- sensors/: acoustic, cbrn, ew, sensor
- entities/: bso, civilian, platform
- intelligence/: assessment, decomposition, fusion, intent, terrain
- operations/: dispatch, logistics, simulation
- governance/: decision-tree, policy
- integrations/: export, nato-coalition, ui"
```

---

### Task 2: Update workspace Cargo.toml

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: Replace members glob**

Change:
```toml
members = ["hello-*"]
```
To:
```toml
members = [
    "sensors/*",
    "entities/*",
    "intelligence/*",
    "operations/*",
    "governance/*",
    "integrations/*",
]
```

- [ ] **Step 2: Verify cargo can resolve workspace**

```bash
cargo check --workspace 2>&1 | head -20
```
Expected: No errors, cargo successfully discovers all crates

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml && git commit -m "chore: update workspace members glob for hierarchical layout"
```

---

### Task 3: Update README.md

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Reorganize examples table into category sections**

Replace the flat table with categorized sections:

```markdown
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
| `hello-nato-coalition` | ModuleHandle / SecurityContext | Coalition-labelled module lifecycle and audit demo |
| `hello-ui` | (UI components) | SolidJS UI plugins |
```

- [ ] **Step 2: Also update the quickstart section to show diversity**

Change:
```
cargo run -p hello-decomposition
cargo run -p hello-policy
cargo run -p hello-simulation
```
To:
```suggestion
# Pick a category and run any example:
cargo run -p hello-sensor      # sensors
cargo run -p hello-policy      # governance
cargo run -p hello-simulation  # operations
```

- [ ] **Step 3: Commit**

```bash
git add README.md && git commit -m "docs: reorganize examples table into category sections"
```

---

### Task 4: Update CONTRIBUTING.md

**Files:**
- Modify: `CONTRIBUTING.md`

- [ ] **Step 1: Update example adding instructions**

In the "Adding a New Example" section, change:
```
3. Register it in the workspace `Cargo.toml` if needed (the glob `hello-*` auto-discovers)
```
To:
```
3. Register it in the workspace `Cargo.toml` if needed (per-category globs auto-discover — add to the appropriate category in `members`)
```

Also update the template copy instruction:
```
1. Copy an existing hello-* crate (e.g. `hello-sensor`) as a template
```
To:
```
1. Copy an existing crate (e.g. `sensors/hello-sensor`) as a template and place it in the matching category directory
```

- [ ] **Step 2: Commit**

```bash
git add CONTRIBUTING.md && git commit -m "docs: update contributing guide for hierarchical layout"
```

---

### Task 5: Verify everything works

- [ ] **Step 1: Full workspace check**

```bash
cargo check --workspace
```
Expected: Compiles without errors

- [ ] **Step 2: Run tests**

```bash
export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0
cargo test --release --workspace
```
Expected: All tests pass

- [ ] **Step 3: Run clippy**

```bash
cargo clippy --workspace -- -D warnings
```
Expected: No warnings

- [ ] **Step 4: Spot-check a few examples run**

```bash
cargo run -p hello-sensor 2>&1 | head -5
cargo run -p hello-policy 2>&1 | head -5
cargo run -p hello-simulation 2>&1 | head -5
```
Expected: Each prints its demo output and exits
