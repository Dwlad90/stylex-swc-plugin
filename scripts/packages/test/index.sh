#!/bin/bash

# Define the patterns: #[test], test_transform(, or test!(
# We use -E for extended regex to use the OR (|) operator
PATTERNS="#\[test\]|test_transform\(|test!\("
script_dir="$(cd "$(dirname "$0")" && pwd)"
workspace_root="$(cd "$script_dir/../../.." && pwd)"
crate_slug="$(basename "$PWD" | tr -c '[:alnum:]_-' '_')"
crate_target_dir="${workspace_root}/target/test-${crate_slug}"

# Search recursively in the src directory
# -q: quiet mode (don't output matches, just exit status)
# -E: Extended regexp
# -r: Recursive
if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
    #Add arguments from call command
    args=("$@")

    NODE_ENV="test" cargo test --target-dir "$crate_target_dir" --lib --bins --tests --all-features "${args[@]}"
    NODE_ENV="test" cargo test --target-dir "$crate_target_dir" --doc --all-features "${args[@]}"
else
    exit 0
fi
