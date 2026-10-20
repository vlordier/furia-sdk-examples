# Engineering guidance for Furia SDK examples

## Purpose and ownership

- This is a Rust workspace of small, independently runnable examples for
  `furia-sdk` capabilities. Keep each example in the package that demonstrates
  that capability; do not move application behavior into this workspace.
- Each example should make its SDK trait implementation, inputs, outputs, and
  observable behavior easy to follow. Avoid helper frameworks that hide the
  trait boundary being taught.
- Share code only when implementations have the same semantics and a shared
  helper makes the example clearer. Similar syntax alone is not a reason to
  couple separate domain examples.
- `furia-sdk` is declared centrally in the workspace. Keep local development on
  the configured Core path dependency and release examples on the documented
  tagged dependency; do not mix versions across member crates.

## Rust quality

- Check trait signatures and model semantics against the configured SDK source
  before changing an implementation. Preserve units, coordinate conventions,
  enum meanings, and error behavior.
- Use explicit types and domain names. Validate external or user-controlled
  values at the boundary and return meaningful errors where the SDK contract
  permits them.
- Do not add `unwrap`, `expect`, `todo!`, or lint suppressions to production
  paths. An executable example may use `expect` for a fixed, valid fixture only
  when the message explains that invariant.
- Keep dependencies minimal and declare test-only dependencies under
  `[dev-dependencies]`. Update `Cargo.lock` when dependencies change.

## Tests and documentation

- Test behavior a reader relies on: boundary inputs, state transitions,
  serialization contracts, and failure cases. Avoid smoke tests that only prove
  a method was called or that a result is nonempty.
- Keep examples runnable and update their README when usage or SDK contracts
  change. Do not silently change units or the meaning of a demonstration.
- Before committing Rust changes, run:

  ```bash
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace --all-targets
  ```

  If a command is blocked by an unrelated workspace failure, record the exact
  failure and run the narrowest relevant package check; do not claim the blocked
  gate passed.

## Change workflow

- Read the relevant example and its SDK trait before editing. Search for an
  existing implementation before adding another one.
- Track substantive work in Beads before editing. Keep commits small and scoped
  to one behavior or refactor, and stage only the files for that change.
- Preserve other worktree changes. Do not force-push, discard user changes, or
  bypass failing checks without an explicit, documented reason.
