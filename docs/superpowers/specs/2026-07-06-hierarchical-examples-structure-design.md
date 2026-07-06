# Hierarchical Examples Structure Design

## Goal

Restructure the flat `hello-*` crate layout into a hierarchical directory tree so related examples live together without changing package names or breaking existing workflows.

## Design

### Directory layout

All `hello-*` crates move under six category directories at the workspace root:

```
sensors/
  hello-acoustic/
  hello-cbrn/
  hello-ew/
  hello-sensor/

entities/
  hello-bso/
  hello-civilian/
  hello-platform/

intelligence/
  hello-assessment/
  hello-decomposition/
  hello-fusion/
  hello-intent/
  hello-terrain/

operations/
  hello-dispatch/
  hello-logistics/
  hello-simulation/

governance/
  hello-decision-tree/
  hello-policy/

integrations/
  hello-export/
  hello-nato-coalition/
  hello-ui/
```

### Package names

Package names remain `hello-acoustic`, `hello-bso`, etc. — no rename. This preserves all `cargo run -p hello-*` commands.

### Workspace config

The root `Cargo.toml` `members` glob `"hello-*"` is replaced with per-category globs:

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

### Files to update

| File | Change |
|---|---|
| `Cargo.toml` | `members` glob → per-category globs |
| `README.md` | Update example table paths (remove relative path changes — all `cargo run` still works) |
| `CONTRIBUTING.md` | Update references to flat structure |
| `CHANGELOG.md` | Already lists flat names — no change needed (package names unchanged) |
| `.github/ISSUE_TEMPLATE/bug_report.md` | Reference stays generic (`cargo run -p hello-...`) — no change |

### Scope

- Crate directories only. No source code changes.
- No CI/CD changes (commands reference package names, not paths).
- README.md files inside each crate reference their own directory only — no change needed.
