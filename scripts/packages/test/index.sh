#!/bin/bash

# The per-crate Rust test runner, reached through `scripty` when a crate points
# its `test` script at it. No crate does today: every crate prints a skip line
# because the Rust suites run once for the whole workspace from
# `pnpm test:crates:workspace`. The script is kept for a direct run from a crate
# directory, and for the crate that points `test` back at `scripty`. Its
# siblings `coverage.sh` and `flamegraph.sh` are reached by `test:coverage` and
# `test:flamegraph`.

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

    #Add arguments from call command
    args=("$@")

    NODE_ENV="test" cargo nextest run "${common_args[@]}" "${args[@]}"
    NODE_ENV="test" cargo test "${common_args[@]}" --doc "${args[@]}"
else
    exit 0
fi
