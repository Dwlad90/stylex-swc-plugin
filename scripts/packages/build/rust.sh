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

  # The `name` key of one manifest section. Both `[package]` and `[[bin]]` hold
  # one, so each is read inside its own section: a plain `grep '^name'` would
  # match either, and both where a manifest declares both.
  manifest_name() {
    awk -v want="$1" '
      /^\[/ { section = $0 }
      section == want && /^name[[:space:]]*=/ {
        sub(/^name[[:space:]]*=[[:space:]]*"/, "")
        sub(/".*$/, "")
        print
        exit
      }
    ' Cargo.toml
  }

  cargo_name=$(manifest_name "[package]")

  # Cargo names a binary after its `[[bin]]` entry when the manifest declares
  # one, and after the package otherwise. A library always uses the package
  # name, so only the binary branch below consults this.
  bin_name=$(manifest_name "[[bin]]")
  : "${bin_name:=$cargo_name}"

  mkdir -p ./dist || handle_error "Failed to create the dist directory"

  if [ ! -f "src/lib.rs" ]; then

    # Build the Rust application if there is no src/lib.rs file
    cargo build --release $verbose_flag || handle_error "Failed to build the Rust project"

    built_path="$(find ../../target/release/"${bin_name}" -type f | tail -1)"

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
