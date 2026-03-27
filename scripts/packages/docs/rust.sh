#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

script_dir="$(cd "$(dirname "$0")" && pwd)"

open_docs=""

for arg in "$@"; do
  if [ "$arg" = "--open" ]; then
    open_docs="--open"
    # shellcheck disable=SC3060
    set -- "${@/--open/}"
  fi
done

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
  export RUSTDOCFLAGS="--enable-index-page -Zunstable-options"

  cargo doc +nightly --no-deps --document-private-items $verbose_flag $open_docs || handle_error "Failed to build cargo docs"
fi
