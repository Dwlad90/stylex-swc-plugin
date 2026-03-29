#!/bin/bash
set -e # Exit immediately if a command fails

# Define the patterns: #[test], test_transform(, or test!(
# We use -E for extended regex to use the OR (|) operator
PATTERNS="#\[test\]|test_transform\(|test!\("

# Search recursively in the src directory
# -q: quiet mode (don't output matches, just exit status)
# -E: Extended regexp
# -r: Recursive
if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
    #Add artuments from call command
    args=("$@")

    # Path resolver is single-threaded
    if [[ "$PWD" == *"stylex-path-resolver"* ]]; then
        args=(-- --test-threads=1)
    fi

    NODE_ENV="test" cargo test --lib --bins --tests --all-features "${args[@]}"
    NODE_ENV="test" cargo test --doc --all-features "${args[@]}"
else
    exit 0
fi
