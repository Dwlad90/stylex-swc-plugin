#!/bin/bash

set -euo pipefail

script_dir="$(cd -P "$(dirname "$0")" && pwd -P)"
# shellcheck source=scripts/packages/test/lib/crate.sh
. "$script_dir/lib/crate.sh"

crate_name="${PWD##*/}"

# Kept in step with the two workspace lists in `package.json` and
# `scripts/coverage-missing.sh`. This list holds crate directory names, so a
# name can differ from the Cargo package name by more than the hyphens:
# stylex-rs-compiler is the crate stylex_compiler_rs. Why each crate is off the
# gate, and which rows a ticket removes, is in "Excluded from Coverage" in
# guidelines/STRUCTURE.md.
case "$crate_name" in
  stylex-evaluator|stylex-logs|stylex-rs-compiler|stylex-state|stylex-test-parser|stylex-transform)
    exit 0
    ;;
esac

if crate_has_tests "$CRATE_TEST_MARKERS"; then
  if [ ! -f "src/lib.rs" ]; then
    exit 0
  fi

  workspace_root="$(crate_workspace_root)"
  crate_target_dir="${workspace_root}/target/coverage-$(crate_slug)"

  IGNORE_REGEX="(tests?|benches?|examples)/"

  NODE_ENV="test" CARGO_TARGET_DIR="$crate_target_dir" cargo +nightly llvm-cov nextest \
    --all-features \
    --fail-uncovered-lines 0 \
    --fail-uncovered-regions 0 \
    --ignore-filename-regex "$IGNORE_REGEX" \
    "$@"
else
  exit 0
fi
