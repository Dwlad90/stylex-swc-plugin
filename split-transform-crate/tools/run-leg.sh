#!/bin/bash
# Run one bench leg in its own target directory, against the saved
# `parent-clean` criterion baseline.
#
# Usage: run-leg.sh <worktree-dir> <target-dir>
set -euo pipefail

worktree="$1"
target="$2"
root="/Users/vladislavbuinovski/Projects/Facebook/stylex-swc-plugin.git"

# Seed the leg's criterion directory with the parent baselines. Copied, never
# shared: a shared target directory serves one leg's rlibs to the other.
mkdir -p "$target"
rm -rf "$target/criterion"
cp -R "$root/bench-13-criterion/parent-leg" "$target/criterion"

cd "$worktree"

# The three evaluator benches live in stylex-transform before ticket 13 and in
# stylex-evaluator after it, so ask cargo which package owns each target.
run_bench() {
  local bench="$1"
  local pkg
  for pkg in stylex_transform stylex_evaluator; do
    if [ -f "crates/${pkg//_/-}/benches/${bench}.rs" ]; then
      CARGO_TARGET_DIR="$target" cargo bench -p "$pkg" --bench "$bench" -- \
        --baseline parent-clean --sample-size 20 --warm-up-time 2 \
        --measurement-time 4 --noplot
      return
    fi
  done
  echo "error: no package owns bench $bench" >&2
  exit 1
}

for bench in concatenation_chain_bench engine_fold_bench evaluate_bench \
             evaluate_depth_bench module_path_bench; do
  run_bench "$bench"
done
