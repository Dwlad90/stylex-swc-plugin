#!/bin/bash

# Fails when the installed dependency graph contains any TypeScript other than
# the single approved exact version.
#
# TypeScript 7 removed the JavaScript compiler API. Any tool that still reaches
# for it (ts-jest, tsup declaration emit, vue-tsc, vite-plugin-dts,
# react-docgen-typescript, @swc-node/register, typescript-eslint) drags a
# TypeScript 5/6 copy back into the graph, which is how those tools keep
# working silently while the repository believes it is on TypeScript 7 only.
# This guard makes that regression loud.

# Exit immediately when any subprocess returns a non-zero command
set -e

EXPECTED_VERSION="7.0.2"

echo "Checking the installed TypeScript graph (expecting only ${EXPECTED_VERSION})..."

# Every `typescript@<version>:` snapshot key declared in the lockfile.
installed=$(
  grep -oE '^  typescript@[0-9]+\.[0-9]+\.[0-9]+:' pnpm-lock.yaml |
    sed -E 's/^  typescript@//; s/:$//' |
    sort -u
)

if [ -z "$installed" ]; then
  echo "error: found no TypeScript entries in pnpm-lock.yaml — is the lockfile present?"
  exit 1
fi

unexpected=$(echo "$installed" | grep -v "^${EXPECTED_VERSION}$" || true)

if [ -n "$unexpected" ]; then
  echo ""
  echo "error: unexpected TypeScript version(s) in the installed graph:"
  echo "$unexpected" | sed 's/^/  - /'
  echo ""
  echo "Only ${EXPECTED_VERSION} is approved. Find the consumer with:"
  echo "  pnpm why typescript -r"
  echo ""
  echo "A version below 7 almost always means some tool still depends on the"
  echo "removed JavaScript TypeScript compiler API. Replace that tool rather"
  echo "than pinning an older TypeScript alongside it."
  exit 1
fi

echo "Installed TypeScript graph contains only ${EXPECTED_VERSION}."
exit 0
