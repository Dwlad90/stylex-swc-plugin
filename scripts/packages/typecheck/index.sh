#!/usr/bin/env sh

# Exit immediately when any subprocess returns a non-zero command
set -e

# No traps here on purpose. This script runs `tsc` and `cargo check`
# synchronously and backgrounds nothing, so there is nothing to clean up, and
# `set -e` already propagates a non-zero status. The previous
# `trap 'kill 0' EXIT` signalled the whole process group — which, when the
# script is run directly rather than through Turbo, includes the caller's
# interactive shell. Running `pnpm typecheck` inside a package killed the
# user's terminal and discarded the compiler diagnostics along with it.

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
  # Prefer `tsconfig.test.json` when the package has one: it is the only config
  # here whose `include` covers `__tests__`. `tsconfig.typecheck.json` extends
  # `tsconfig.json`, which is scoped to `src/**` because it also drives the
  # build, so on its own it type-checks the sources and silently skips every
  # test file.
  #
  # Guarded rather than silently preferred: a package that has tests but no
  # `tsconfig.test.json` would otherwise fall back and look checked while its
  # suite went unread, which is the failure this change exists to remove.
  tsconfig_name="tsconfig.typecheck.json"

  if [ -f "tsconfig.test.json" ]; then
    tsconfig_name="tsconfig.test.json"
  elif [ -d "__tests__" ] || [ -d "__test__" ]; then
    echo "Package has tests but no tsconfig.test.json; its suite would not be type-checked."
    echo "Add tsconfig.test.json (extend ./tsconfig.json, include __tests__)."
    exit 1
  fi

  if [ ! -f "${tsconfig_name}" ]; then
    echo "${tsconfig_name} not found at ${tsconfig_name}"
    exit 1
  fi

  tsc --noEmit --emitDeclarationOnly false --declarationMap false -p "${tsconfig_name}"

fi;

if [ "$rust" = true ]; then
  cargo check --all-targets --all-features
fi;
