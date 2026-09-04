#!/bin/bash

# Stop at the first command that fails. Without this the status of the script
# was the status of the doc run below, and a red `cargo nextest run` before it
# reported a pass.
#
# `-u` needs the arguments to stay as "$@". Bash 3.2, which macOS ships, stops
# with an error for an empty array, so a copy such as `args=("$@")` breaks
# every run that gives no argument.
set -euo pipefail

# The per-crate Rust test runner, reached through `scripty` when a crate points
# its `test` script at it. No crate does today: 23 of the 24 print a skip line
# because the Rust suites run once for the whole workspace from
# `pnpm test:crates:workspace`, and `stylex-rs-compiler` runs `vitest` for its
# JavaScript suite. The script is kept for a direct run from a crate directory.
# Its siblings `coverage.sh` and `flamegraph.sh` are reached by `test:coverage`
# and `test:flamegraph`.

script_dir="$(cd -P "$(dirname "$0")" && pwd -P)"
# shellcheck source=scripts/packages/test/lib/crate.sh
. "$script_dir/lib/crate.sh"

workspace_root="$(crate_workspace_root)"
crate_target_dir="${workspace_root}/target/test-$(crate_slug)"

# This runner also reads the module gate, which the other two scripts do not.
if crate_has_tests "$CRATE_TEST_MARKERS_WITH_GATE"; then
    # Common arguments for all tests. An array rather than a string so the
    # target directory survives a path containing spaces instead of being split
    # into two arguments.
    common_args=(--target-dir "$crate_target_dir" --all-features)

    NODE_ENV="test" cargo nextest run "${common_args[@]}" "$@"
    NODE_ENV="test" cargo test "${common_args[@]}" --doc "$@"
else
    exit 0
fi
