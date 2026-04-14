#!/bin/bash

set -euo pipefail

PATTERNS="#\[test\]|test_transform\(|test!\("
crate_name="$(basename "$PWD")"

case "$crate_name" in
  stylex-logs|stylex-rs-compiler|stylex-test-parser|stylex-transform|stylex-css-parser)
    exit 0
    ;;
esac

if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
  if [ ! -f "src/lib.rs" ]; then
    exit 0
  fi

  script_dir="$(cd "$(dirname "$0")" && pwd)"
  workspace_root="$(cd "$script_dir/../../.." && pwd)"
  crate_slug="$(basename "$PWD" | tr -c '[:alnum:]_-' '_')"
  crate_target_dir="${workspace_root}/target/coverage-${crate_slug}"

  IGNORE_REGEX="(tests?|benches?|examples)/"

  NODE_ENV="test" cargo +nightly llvm-cov nextest \
    --target-dir "$crate_target_dir" \
    --all-features \
    --fail-uncovered-lines 0 \
    --fail-uncovered-regions 0 \
    --fail-under-functions 0 \
    --ignore-filename-regex "$IGNORE_REGEX" \
    "$@"
else
  exit 0
fi
