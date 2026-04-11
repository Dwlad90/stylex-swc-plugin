#!/bin/bash

# Define the patterns: #[test], test_transform(, or test!(
PATTERNS="#\\[test\\]|test_transform\\(|test!\\("

# Run flamegraph only for crates that actually declare tests.
if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
  args=("$@")
  NODE_ENV="test" cargo flamegraph --root --test "${args[@]}"
else
  exit 0
fi
