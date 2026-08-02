#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

pids=""

# Terminate only the children this script started.
#
# This replaces `kill 0`, which signals every process in the group — including
# the caller's interactive shell when the script is run directly rather than
# through Turbo, killing the user's terminal along with the build. Tracking the
# PIDs we spawned gives the same guarantee (no orphaned compilers when one side
# fails) without reaching outside this script.
#
# Reached on failure via `set -e` and on interrupt via the signal traps; on a
# clean run the children have already been reaped, so the `kill` is a harmless
# no-op. The EXIT trap must not itself exit, so the script's real status is
# preserved.
cleanup() {
  [ -n "$pids" ] || return 0
  # shellcheck disable=SC2086
  kill $pids 2>/dev/null || true
  return 0
}

trap 'cleanup; exit 130' INT
trap 'cleanup; exit 143' TERM
trap cleanup EXIT

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# Build js and types concurrently
"$script_dir/rust.sh" "$@" & pids="${pids}$! "
"$script_dir/typescript.sh" "$@" & pids="${pids}$! "


# Exit with correct exit code if either one fails
for pid in $pids; do
  wait "$pid"
done