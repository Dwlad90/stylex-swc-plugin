#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# No traps, and no backgrounding. This script runs exactly one child, so there
# is no sibling that could be left orphaned when it fails — the concurrency
# scaffolding here was copied from `build/index.sh`, which really does run two.
# `set -e` propagates the child's status, which is all the `& … wait` pair was
# achieving.

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

"$script_dir/rust.sh" "$@"
