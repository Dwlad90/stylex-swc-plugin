#!/bin/bash

# Stop at the first command that fails, as `index.sh` and `coverage.sh` do.
# Without this the status of the script is the status of the last command, and
# a red `cargo flamegraph` before it can report a pass.
#
# `-u` needs the arguments to stay as "$@". Bash 3.2, which macOS ships, stops
# with an error for an empty array, so a copy such as `args=("$@")` breaks
# every run that gives no argument.
set -euo pipefail

script_dir="$(cd -P "$(dirname "$0")" && pwd -P)"
# shellcheck source=scripts/packages/test/lib/crate.sh
. "$script_dir/lib/crate.sh"

# Run flamegraph only for crates that actually declare tests.
if crate_has_tests "$CRATE_TEST_MARKERS"; then
  NODE_ENV="test" cargo flamegraph --root --test "$@"
else
  exit 0
fi
