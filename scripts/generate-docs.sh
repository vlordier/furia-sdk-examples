#!/usr/bin/env bash
set -euo pipefail
# Auto-generate README.md and docs/examples.md from Cargo.toml descriptions.
# Usage: ./scripts/generate-docs.sh

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
README="$ROOT/README.md"
DOCS_EXAMPLES="$ROOT/docs/examples.md"

# Extract crate info from Cargo.toml: output "name|trait|description"
parse_crate() {
  local cargo_toml="$1"
  local name desc
  name=$(grep -E '^name\s*=' "$cargo_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
  desc=$(grep -E '^description\s*=' "$cargo_toml" | head -1 | sed 's/.*"\(.*\)"/\1/')

  # Strip "Hello World: " prefix
  desc="${desc#Hello World: }"

  # Extract trait name and description
  if [[ "$desc" =~ ^implementing\ ([A-Za-z]+)\ —\ (.+)$ ]]; then
    trait="${BASH_REMATCH[1]}"
    short_desc="${BASH_REMATCH[2]}"
  elif [[ "$desc" =~ ^([A-Za-z]+)\ —\ (.+)$ ]]; then
    trait="${BASH_REMATCH[1]}"
    short_desc="${BASH_REMATCH[2]}"
  elif [[ "$desc" =~ ^(Example|composing|using) ]]; then
    trait=""
    short_desc="$desc"
  else
    trait=""
    short_desc="$desc"
  fi

  echo "$name|$trait|$short_desc"
}

generate_tables() {
  for dir in sensors entities intelligence operations governance integrations cuas; do
    case "$dir" in
      sensors)       heading="Sensors" ;;
      entities)      heading="Entities" ;;
      intelligence)  heading="Intelligence" ;;
      operations)    heading="Operations" ;;
      governance)    heading="Governance" ;;
      integrations)  heading="Integrations" ;;
      cuas)          heading="C-UAS / Air Defence" ;;
    esac

    echo ""
    echo "### $heading"
    echo ""
    echo "| Crate | SDK Trait | Description |"
    echo "|-------|-----------|-------------|"

    for cargo_toml in $(find "$ROOT/$dir" -name Cargo.toml -maxdepth 2 | sort); do
      IFS='|' read -r crate_name crate_trait crate_desc <<< "$(parse_crate "$cargo_toml")"
      echo "| \`$crate_name\` | $crate_trait | $crate_desc |"
    done
  done
}

# Generate README.md
{
  echo "# Furia SDK Examples"
  echo ""
  echo "Standalone example binaries for the Furia SDK traits — reference patterns for C2 plugin developers."
  echo ""
  echo "## Quickstart"
  echo ""
  echo '```bash'
  echo "# Pick a category and run any example:"
  echo "cargo run -p hello-sensor      # sensors"
  echo "cargo run -p hello-policy      # governance"
  echo "cargo run -p hello-simulation  # operations"
  echo ""
  echo "# Run all tests"
  echo "export CARGO_TARGET_DIR=.cargo-target CARGO_INCREMENTAL=0"
  echo "cargo test --release --workspace"
  echo '```'
  echo ""
  echo "## Examples"
  generate_tables
  echo ""
  echo "## Architecture"
  echo ""
  echo "Each example is a standalone binary that:"
  echo "1. Implements or exercises one SDK trait (or related shared type family) with a local demo struct"
  echo "2. Calls the trait methods directly in a minimal CLI program and unit tests"
  echo "3. Uses \`ModuleHandle\` where the trait requires SDK context"
  echo "4. Avoids platform registration so examples stay small and runnable without a host process"
  echo ""
  echo "See also: [furia-plugin-example](https://github.com/vlordier/furia-plugin-example) (complete plugin), [furia-core](https://github.com/vlordier/furia-core) (SDK traits)."
  echo ""
} > "$README"

# Generate docs/examples.md (for mkdocs)
{
  echo "# Examples Catalog"
  echo ""
  echo "All {{ crate_count }} example crates organized by category."
  echo ""
  generate_tables
  echo ""
  echo "## See Also"
  echo ""
  echo "- [Architecture](architecture.md)"
  echo ""
} > "$DOCS_EXAMPLES"

# Replace placeholder with actual count
CRATE_COUNT=$(find "$ROOT" -name Cargo.toml -maxdepth 3 \
  | grep -v target | grep -v ".cargo" | grep -v "/\." | wc -l | tr -d ' ')
sed -i '' "s/{{ crate_count }}/$CRATE_COUNT/g" "$DOCS_EXAMPLES"

echo "Regenerated $README and $DOCS_EXAMPLES"
