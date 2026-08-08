#!/bin/bash

# Exit immediately when any subprocess returns a non-zero command
set -e

echo "Checking dependency versions across workspace..."

# Addressed by path, not `pnpm exec` -- see the `Gotchas` section of
# guidelines/git/HOOKS.md. Both callers, the hook and CI, run from the
# repository root.
if ./node_modules/.bin/syncpack lint; then
    echo "All dependencies are in sync across the workspace."
    exit 0
fi

echo ""
echo "Please run 'pnpm syncpack fix' to fix the mismatches or fix them manually."
exit 1
