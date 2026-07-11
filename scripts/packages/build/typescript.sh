#!/usr/bin/env sh

script_dir="$(cd "$(dirname "$0")" && pwd)"

# shellcheck disable=SC1091
. "$script_dir"/../../functions.sh

# shellcheck disable=1091
. "$script_dir"/../../parse-args.sh
: "${build_ts:=false}"
: "${flatten:=false}"

if [ "$build_ts" = true ]; then
  tsconfig_name="tsconfig.build.json"

  if [ ! -f "${tsconfig_name}" ]; then
    echo "${tsconfig_name} not found at ${tsconfig_name}"
    exit 1
  fi

  pnpm tsc -p "${tsconfig_name}"|| handle_error "Failed to build the TypeScript project"

  if [ "$flatten" = true ]; then
    # Move all files from src to dist and remove src directory
    if [ -d dist/src ]; then
      mkdir -p dist
      mv dist/src/* dist/ || handle_error "Failed to move files from src to dist"
      rmdir dist/src || handle_error "Failed to remove src directory"
    fi
  fi

  # Copy CSS assets shipped alongside compiled sources (carrier/dummy stylesheets)
  for css_file in src/*.css; do
    if [ -f "$css_file" ]; then
      mkdir -p dist
      cp "$css_file" dist/ || handle_error "Failed to copy $css_file"
    fi
  done
fi
