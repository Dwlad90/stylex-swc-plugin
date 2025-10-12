#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

ts=false;
rust=false;

while [ "$#" -ne 0 ]; do
  case "$1" in
  -ts | --ts | -typescript | --typescript)
    ts=true
    shift
    ;;
  -rust | --rust | -rs | --rs)
    rust=true
    shift
    ;;
  esac
done

if [ "$ts" = true ]; then
  tsconfig_name="tsconfig.typecheck.json"

  if [ ! -f "${tsconfig_name}" ]; then
    echo "${tsconfig_name} not found at ${tsconfig_name}"
    exit 1
  fi

  tsc --noEmit --emitDeclarationOnly false --declarationMap false -p "${tsconfig_name}"

fi;

if [ "$rust" = true ]; then
  cargo check --all-targets --all-features
fi;

# Remove traps to exit with 0
trap - INT TERM EXIT
