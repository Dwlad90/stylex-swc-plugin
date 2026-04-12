#!/bin/bash

# Define the patterns: #[test], test_transform(, or test!(
PATTERNS="#\\[test\\]|test_transform\\(|test!\\("
script_dir="$(cd "$(dirname "$0")" && pwd)"
workspace_root="$(cd "$script_dir/../../.." && pwd)"
tarpaulin_config="$workspace_root/tarpaulin.toml"
crate_slug="$(basename "$PWD" | tr -c '[:alnum:]_-' '_')"
crate_target_dir="${workspace_root}/target/tarpaulin-${crate_slug}"
crate_rel_path="${PWD#"${workspace_root}"/}"
crate_include_glob="${crate_rel_path}/src/**/*.rs"
crate_name="$(basename "$PWD")"
extra_exclude_args=()

case "$crate_name" in
  stylex-logs|stylex-rs-compiler|stylex-test-parser|stylex-transform|stylex-css-parser)
    exit 0
    ;;
esac

if [ "$crate_name" = "stylex-css" ]; then
  extra_exclude_args+=(--exclude-files "crates/stylex-css/src/values/parser.rs")
fi

# Run coverage only for crates that actually declare tests.
if grep -qRE --include="*.rs" "$PATTERNS" src tests; then
  # Tarpaulin reporting is configured around library crates; skip bin-only packages.
  if [ ! -f "src/lib.rs" ]; then
    exit 0
  fi

  NODE_ENV="test" cargo tarpaulin --config "$tarpaulin_config" --target-dir "$crate_target_dir" --include-files "$crate_include_glob" "${extra_exclude_args[@]}" --skip-clean "$@"
else
  exit 0
fi
