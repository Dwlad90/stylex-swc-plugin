#!/bin/bash

# Exit immediately when any subprocess returns a non-zero command
set -e

echo "Checking dependency versions across workspace..."

if pnpm exec syncpack lint; then
    echo "All dependencies are in sync across the workspace."
    exit 0
fi

echo ""
echo "Please run 'pnpm syncpack fix' to fix the mismatches or fix them manually."
exit 1
