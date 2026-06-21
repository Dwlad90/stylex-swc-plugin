#!/usr/bin/env bash
# scripts/coverage-missing.sh
#
# Pinpoint uncovered lines in the Rust workspace using cargo-llvm-cov.
#
# This is the diagnostic companion to `pnpm test:coverage:workspace`, which only
# reports pass/fail percentages per file. This script additionally prints the
# exact `file: line` list of code that no test exercises, so you know precisely
# which branches still need a test.
#
# REQUIREMENTS
#   - Rust nightly toolchain          : rustup toolchain install nightly
#   - cargo-llvm-cov + cargo-nextest  : cargo install cargo-llvm-cov cargo-nextest --locked
#
# USAGE
#   scripts/coverage-missing.sh                 # whole workspace (matches CI excludes)
#   scripts/coverage-missing.sh stylex_css      # single crate (fast iteration)
#   scripts/coverage-missing.sh -p stylex_css   # same, explicit flag
#   scripts/coverage-missing.sh --html          # also write an HTML report, print its path
#   scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
#   scripts/coverage-missing.sh -h | --help
#
# EXIT STATUS
#   0  every measured line is covered
#   1  one or more lines are uncovered (the `file: line` list is printed above)

set -euo pipefail

# Crates excluded from workspace coverage, kept in sync with the
# `test:coverage:workspace` script in the root package.json.
WORKSPACE_EXCLUDES=(
  --exclude stylex_logs
  --exclude stylex_compiler_rs
  --exclude stylex_test_parser
  --exclude stylex_css_parser
  --exclude stylex_transform
)

# Generated test/bench/example files are never counted, matching CI.
IGNORE_REGEX='(tests?|benches?|examples)/'

usage() {
  cat <<'EOF'
coverage-missing.sh — pinpoint uncovered lines in the Rust workspace.

USAGE
  scripts/coverage-missing.sh                 # whole workspace (matches CI excludes)
  scripts/coverage-missing.sh stylex_css      # single crate (fast iteration)
  scripts/coverage-missing.sh -p stylex_css   # same, explicit flag
  scripts/coverage-missing.sh --html          # also write an HTML report, print its path
  scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
  scripts/coverage-missing.sh -h | --help

EXIT STATUS
  0  every measured line is covered
  1  one or more lines are uncovered (the `file: line` list is printed above)
EOF
  exit "${1:-0}"
}

package=""
html=0
open=0

while [ $# -gt 0 ]; do
  case "$1" in
    -h | --help) usage 0 ;;
    --html) html=1 ;;
    --open)
      html=1
      open=1
      ;;
    -p | --package)
      [ $# -ge 2 ] || { echo "error: $1 requires a crate name" >&2; exit 2; }
      package="$2"
      shift
      ;;
    -*) echo "error: unknown option '$1' (try --help)" >&2; exit 2 ;;
    *)
      if [ -n "$package" ]; then
        echo "error: unexpected argument '$1'" >&2
        exit 2
      fi
      package="$1"
      ;;
  esac
  shift
done

# Select scope: a single crate (fast) or the whole workspace (CI parity).
# Arrays keep each argument a distinct word, so no `shellcheck disable=SC2086`.
scope=()
if [ -n "$package" ]; then
  scope=(-p "$package")
  echo "==> Coverage for crate: $package"
else
  scope=(--workspace "${WORKSPACE_EXCLUDES[@]}")
  echo "==> Coverage for workspace (excluding non-instrumented crates)"
fi

# `--show-missing-lines` prints the uncovered `file: line` list.
# `--fail-uncovered-lines 0` makes the command exit non-zero if any remain,
# so this script is CI-friendly and usable as a pre-commit gate.
report_flags=(
  --show-missing-lines
  --fail-uncovered-lines 0
  --fail-uncovered-regions 0
  --fail-under-functions 0
)

if [ "$html" -eq 1 ]; then
  report_flags+=(--html)
  [ "$open" -eq 1 ] && report_flags+=(--open)
fi

cargo +nightly llvm-cov nextest \
  "${scope[@]}" \
  --all-features \
  --ignore-filename-regex "$IGNORE_REGEX" \
  "${report_flags[@]}"
