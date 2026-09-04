#!/bin/bash

script_dir="$(cd -P "$(dirname "$0")" && pwd -P)"
# shellcheck source=scripts/packages/test/lib/crate.sh
. "$script_dir/lib/crate.sh"

# Run flamegraph only for crates that actually declare tests.
if crate_has_tests "$CRATE_TEST_MARKERS"; then
  args=("$@")
  NODE_ENV="test" cargo flamegraph --root --test "${args[@]}"
else
  exit 0
fi
