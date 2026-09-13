#!/usr/bin/env bash
# Save one criterion baseline for every bench in the workspace, in one session.
#
# The allocator a bench measures is part of its number. Ticket 17 made every
# bench link `swc_malloc`, the allocator the published addon runs, which closed
# every series taken before it. This script opens the next one.
#
# Run it from the code worktree root, on a machine with nothing else on it.
#
# Usage: rebaseline.sh <baseline-name> <log-file>
set -euo pipefail

name="${1:?usage: rebaseline.sh <baseline-name> <log-file>}"
log="${2:?usage: rebaseline.sh <baseline-name> <log-file>}"

# A baseline names the commit it measures, so the tree must be clean. A dirty
# tree makes the record unreproducible and gives no sign of it later.
if [ -n "$(git status --porcelain -- ':!.scratch')" ]; then
  echo "refusing: the worktree has uncommitted changes outside .scratch" >&2
  exit 1
fi

# The machine must not be saturated. Numbers taken under load are noise, and a
# baseline is the one measurement every later comparison is read against. The
# bar is four of ten cores busy, which a desktop with a browser open stays
# under; the failure this guard exists to catch read 0% idle for 19 hours.
idle=$(iostat -c 2 2>/dev/null | tail -1 | awk '{print $(NF-3)}')
if ! [ "$idle" -eq "$idle" ] 2>/dev/null; then
  echo "refusing: cannot read CPU idle from iostat" >&2
  exit 1
fi
if [ "${SKIP_IDLE_CHECK:-0}" != "1" ] && [ "$idle" -lt 60 ]; then
  echo "refusing: the machine is ${idle}% idle, which is too busy to measure" >&2
  echo "close other work, or set SKIP_IDLE_CHECK=1 to measure anyway" >&2
  exit 1
fi

# Ask each manifest which bench targets it owns. Naming the target matters:
# a bare `cargo bench -p <pkg>` also runs the crate's lib test harness as a
# bench target, and that harness rejects criterion's own options.
pairs=$(
  for manifest in crates/*/Cargo.toml; do
    awk '
      /^\[package\]/ { section = "package"; next }
      /^\[\[bench\]\]/ { section = "bench"; next }
      /^\[/ { section = ""; next }
      section && /^[[:space:]]*name[[:space:]]*=/ {
        value = $0
        sub(/^[^=]*=[[:space:]]*/, "", value)
        gsub(/"/, "", value)
        if (section == "package") { package = value } else { print package, value }
      }
    ' "$manifest"
  done
)

{
  echo "criterion baseline '$name'"
  echo "commit:  $(git rev-parse HEAD)"
  echo "subject: $(git log -1 --format=%s)"
  echo "machine: $(sysctl -n machdep.cpu.brand_string), $(sysctl -n hw.ncpu) cores"
  echo "system:  $(sw_vers -productName) $(sw_vers -productVersion)"
  echo "rustc:   $(rustc --version)"
  echo "idle:    ${idle}% before the run"
  echo
} > "$log"

# Build every bench target first, so no target compiles while another one is
# being timed.
while read -r package bench; do
  [ -n "$bench" ] || continue
  cargo bench -p "$package" --bench "$bench" --no-run < /dev/null
done <<< "$pairs"

while read -r package bench; do
  [ -n "$bench" ] || continue
  echo "==> $package $bench" | tee -a "$log"
  cargo bench -p "$package" --bench "$bench" -- \
    --save-baseline "$name" --noplot < /dev/null 2>&1 | tee -a "$log"
done <<< "$pairs"

targets=$(grep -c '^==> ' "$log")
measurements=$(grep -c 'time:   \[' "$log")
echo "saved criterion baseline '$name'"
echo "$targets targets, $measurements measurements; log in $log"
echo "check both against the last run before you trust the numbers"
