#!/usr/bin/env sh

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# shellcheck disable=1091
. "$script_dir"/../../parse-args.sh
: "${build_ts:=false}"

if [ "$build_ts" = true ]; then
  pnpm tsc || handle_error "Failed to build the TypeScript project"
fi