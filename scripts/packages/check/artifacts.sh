#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

artifacts_path="${1:-./dist/index.js}"
artifacts_dir=$(dirname "$artifacts_path")

# Example usage:
echo "Full path: $artifacts_path"
echo "Directory: $artifacts_dir"


ls "$artifacts_dir"
if [ ! -f "$artifacts_path" ]; then
  echo "Artifacts not found at $artifacts_path"
  exit 1
fi

# Remove traps to exit with 0
trap - INT TERM EXIT
