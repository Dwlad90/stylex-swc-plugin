#!/bin/bash
# Re-measure an already-built leg. No rebuild, so the binary is the same one
# the first pass measured: the spread this shows is measurement noise alone.
#
# Usage: rerun-leg.sh <worktree-dir> <target-dir>
set -euo pipefail

worktree="$1"
target="$2"
cd "$worktree"

for bench in concatenation_chain_bench engine_fold_bench evaluate_bench \
             evaluate_depth_bench module_path_bench; do
  for pkg in stylex_transform stylex_evaluator; do
    if [ -f "crates/${pkg//_/-}/benches/${bench}.rs" ]; then
      CARGO_TARGET_DIR="$target" cargo bench -p "$pkg" --bench "$bench" -- \
        --baseline parent-clean --sample-size 20 --warm-up-time 2 \
        --measurement-time 4 --noplot
      break
    fi
  done
done
