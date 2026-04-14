#!/bin/bash

# Define the patterns: #[test], test_transform(, or test!(
# We use -E for extended regex to use the OR (|) operator
PATTERNS="#\[test\]|test_transform\(|test!\(|#\[cfg\(test\)\]"
script_dir="$(cd "$(dirname "$0")" && pwd)"
workspace_root="$(cd "$script_dir/../../.." && pwd)"
crate_slug="$(basename "$PWD" | tr -c '[:alnum:]_-' '_')"
crate_target_dir="${workspace_root}/target/test-${crate_slug}"

# Search recursively in the src directory
# -q: quiet mode (don't output matches, just exit status)
# -E: Extended regexp
# -r: Recursive
if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
    # Common arguments for all tests
    common_args="--target-dir $crate_target_dir --all-features"

    #Add arguments from call command
    args=("$@")

    NODE_ENV="test" cargo nextest run $common_args "${args[@]}"
    NODE_ENV="test" cargo test $common_args --doc "${args[@]}"
else
    exit 0
fi
