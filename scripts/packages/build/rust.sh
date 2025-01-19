#!/usr/bin/env sh

script_dir="$(cd "$(dirname "$0")" && pwd)"
dist_dir="./dist"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# shellcheck disable=1091
. "$script_dir"/../../parse-args.sh
: "${build_rust:=false}"

if [ "$build_rust" = true ]; then

  # Check if running in GitHub Actions CI
  if [ "$GITHUB_ACTIONS" = "true" ]; then
    verbose_flag="--verbose"
  else
    verbose_flag=""
  fi

  cargo_name=$(grep '^name' Cargo.toml | sed 's/name = "\(.*\)"/\1/')

  mkdir -p ./dist || handle_error "Failed to create the dist directory"

  if [ ! -f "src/lib.rs" ]; then

    # Build the Rust application if there is no src/lib.rs file
    cargo build --release $verbose_flag || handle_error "Failed to build the Rust project"

    built_path="$(find ../../target/release/"${cargo_name}" -type f | tail -1)"

    if [ -z "$built_path" ]; then
      handle_error "Could not find a built file"
    fi
  else
    # Build the Rust library if there is a src/lib.rs file
    cargo build --lib --release --target=wasm32-wasip1 $verbose_flag || handle_error "Failed to build the Rust library"

    built_path="$(find ../../target/wasm32-wasip1/release/"${cargo_name}".wasm -type f | tail -1)"

    if [ -z "$built_path" ]; then
      handle_error "No .wasm file found in the target directory"
    fi
  fi

  cp "$built_path" "$dist_dir" || handle_error "Failed to copy the built file"
fi
