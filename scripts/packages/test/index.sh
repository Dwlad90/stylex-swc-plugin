#!/bin/bash

# The per-crate Rust test runner, reached through `scripty` when a crate points
# its `test` script at it. No crate does today: every crate prints a skip line
# because the Rust suites run once for the whole workspace from
# `pnpm test:crates:workspace`. The script is kept for a direct run from a crate
# directory, and for the crate that points `test` back at `scripty`. Its
# siblings `coverage.sh` and `flamegraph.sh` are reached by `test:coverage` and
# `test:flamegraph`.

# Define the patterns: #[test], test_transform(, or test!(
# We use -E for extended regex to use the OR (|) operator
PATTERNS="#\[test\]|test_transform\(|test!\(|#\[cfg\(test\)\]"
script_dir="$(cd "$(dirname "$0")" && pwd)"
workspace_root="$(cd "$script_dir/../../.." && pwd)"
# The crate directory name, with anything a path should not carry folded to an
# underscore. Expanded rather than piped from `basename`, whose trailing newline
# `tr` also folds -- which left every target directory with a stray underscore.
crate_slug="$(printf '%s' "${PWD##*/}" | tr -c '[:alnum:]_-' '_')"
crate_target_dir="${workspace_root}/target/test-${crate_slug}"

# Only the directories this crate has. Most crates keep every test beside the
# code and have no `tests/` at all, and a missing path is an error to grep. That
# error status outranks a match found in the directory beside it, so naming
# `tests` unconditionally made a crate with tests in `src` report none and skip
# its whole suite without a word.
search_dirs=()
for dir in src tests; do
    if [ -d "$dir" ]; then
        search_dirs+=("$dir")
    fi
done

# A crate with neither directory holds no test to run.
if [ ${#search_dirs[@]} -eq 0 ]; then
    exit 0
fi

# Search recursively in the directories collected above
# -q: quiet mode (don't output matches, just exit status)
# -E: Extended regexp
# -R: Recursive
if grep -qRE --include="*.rs" "$PATTERNS" "${search_dirs[@]}"; then
    # Common arguments for all tests. An array rather than a string so the
    # target directory survives a path containing spaces instead of being split
    # into two arguments.
    common_args=(--target-dir "$crate_target_dir" --all-features)

    #Add arguments from call command
    args=("$@")

    NODE_ENV="test" cargo nextest run "${common_args[@]}" "${args[@]}"
    NODE_ENV="test" cargo test "${common_args[@]}" --doc "${args[@]}"
else
    exit 0
fi
