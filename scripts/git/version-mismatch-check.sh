#!/bin/bash

echo "Checking dependency versions across workspace..."

raw_output=$(FORCE_COLOR=true pnpm exec syncpack list-mismatches --source "**/package.json")

last_line=$(echo "$raw_output" | tail -n 1)
has_no_mismatches=0

if [[ "$last_line" == *"already valid"* ]]; then
    has_no_mismatches=1
fi

if [[ $has_no_mismatches -eq 1 ]]; then
    echo "All dependencies are in sync across the workspace."
    exit 0
fi

echo "$raw_output"
echo "Please run 'pnpm syncpack fix-mismatches --source \"**/package.json\"' to fix the mismatches or fix them manually."

exit 1
