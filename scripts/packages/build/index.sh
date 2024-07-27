#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

pids=""

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

# Remove traps to exit with 0
trap - INT TERM EXIT