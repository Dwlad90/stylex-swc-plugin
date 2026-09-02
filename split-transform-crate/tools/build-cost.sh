#!/bin/bash
# Measure the build cost of one crate-type configuration.
#
#   cold       -- empty target directory, so third-party dependencies build too.
#                 Comparable with the cold build recorded in baseline.md.
#   workspace  -- the same tree after only the workspace's own crates are
#                 cleaned, so the dependency build drops out and the crate-type
#                 change is the whole of the difference.
#
# Usage: build-cost.sh <worktree-dir> <target-dir> <label> [--release]
set -euo pipefail

worktree="$1"
target="$2"
label="$3"
shift 3
extra=("$@")

cd "$worktree"
rm -rf "$target"
export CARGO_TARGET_DIR="$target"

echo "### $label ${extra[*]-dev}"

echo "--- cold build"
/usr/bin/time -p cargo build --workspace --all-features ${extra[@]+"${extra[@]}"} 2>&1 | tail -4
echo "--- target size after cold build (MiB)"
du -sm "$target" | cut -f1

# Clean only this workspace's own crates and leave every dependency in place.
members="$(cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys; print(" ".join(p["name"] for p in json.load(sys.stdin)["packages"]))')"
for member in $members; do
  cargo clean -p "$member" ${extra[@]+"${extra[@]}"} >/dev/null 2>&1 || true
done

echo "--- workspace-only rebuild"
/usr/bin/time -p cargo build --workspace --all-features ${extra[@]+"${extra[@]}"} 2>&1 | tail -4
echo "--- dynamic libraries emitted (count, then total KiB)"
find "$target" -name '*.dylib' -not -path '*/deps/*' | wc -l
find "$target" -name '*.dylib' -not -path '*/deps/*' -exec du -k {} + | awk '{t+=$1} END {print t+0}'
echo "--- final target size (MiB)"
du -sm "$target" | cut -f1
