#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# No traps here on purpose. This script only tests for a file and backgrounds
# nothing, so there is nothing to clean up. The previous `trap 'kill 0' EXIT`
# signalled the entire process group, which includes the caller's interactive
# shell whenever the script is run directly rather than through Turbo.

artifacts_path="${1:-./dist/index.js}"

if [ ! -f "$artifacts_path" ]; then
  echo "Artifacts not found at $artifacts_path"
  exit 1
fi
