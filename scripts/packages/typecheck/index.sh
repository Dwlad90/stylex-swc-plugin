#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# Kill all subprocesses when exiting
# shellcheck disable=2154
trap 'exit $exit_code' INT TERM
trap 'exit_code=$?; kill 0' EXIT

tsconfig_name="tsconfig.typecheck.json"

if [ ! -f "${tsconfig_name}" ]; then
  echo "${tsconfig_name} not found at ${tsconfig_name}"
  exit 1
fi

tsc --noEmit --emitDeclarationOnly false --declarationMap false -p "${tsconfig_name}"

# Remove traps to exit with 0
trap - INT TERM EXIT
