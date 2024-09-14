#!/usr/bin/env sh

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

  cargo doc --no-deps $verbose_flag $open_docs || handle_error "Failed to build cargo docs"

  echo "<meta http-equiv=\"refresh\" content=\"0; url=stylex_swc_plugin\">" > ../../target/doc/index.html
fi
