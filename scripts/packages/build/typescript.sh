#!/usr/bin/env sh

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# shellcheck disable=1091
. "$script_dir"/../../parse-args.sh
: "${build_ts:=false}"

if [ "$build_ts" = true ]; then
  tsconfig_name="tsconfig.build.json"

  if [ ! -f "${tsconfig_name}" ]; then
    echo "${tsconfig_name} not found at ${tsconfig_name}"
    exit 1
  fi

  pnpm tsc -p "${tsconfig_name}"|| handle_error "Failed to build the TypeScript project"

  # Copy virtual CSS if exists
  if [ -f "src/stylex.virtual.css" ]; then
    mkdir -p dist
    cp src/stylex.virtual.css dist/ || handle_error "Failed to copy stylex.virtual.css"
  fi
fi
