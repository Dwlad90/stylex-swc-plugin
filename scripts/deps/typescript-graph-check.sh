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

# Exit immediately when any subprocess returns a non-zero command, and treat
# unset variables as errors. `pipefail` is deliberately left off: the `grep`
# below is allowed to match nothing, which is handled explicitly.
set -eu

EXPECTED_VERSION="7.0.2"

# Versions that are knowingly tolerated, each with the tool that requires it.
# An entry here is a statement that the copy is analysis-only and never
# compiles this repository's source; anything not listed fails the check.
#
#   5.6.1-rc  @arethetypeswrong/core, via `attw: true` in
#             packages/unplugin/tsdown.config.ts. ATTW parses published `.d.ts`
#             files using the TypeScript 5 compiler API, which is exactly the
#             API TypeScript 7 removed and has no replacement for yet. It reads
#             build output and never compiles `src`, so it cannot mask a
#             TypeScript 7 incompatibility in this repository's own code.
#
# Remove an entry as soon as its tool ships a TypeScript 7 build.
ALLOWED_EXTRA_VERSIONS="5.6.1-rc"

# Resolve the lockfile from the repository root so the guard behaves the same
# whether it is invoked by a root script, by Turbo, or from a package
# directory.
repo_root=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
lockfile="${repo_root}/pnpm-lock.yaml"

if [ ! -f "$lockfile" ]; then
  echo "error: no lockfile at ${lockfile}"
  exit 1
fi

echo "Checking the installed TypeScript graph (expecting only ${EXPECTED_VERSION})..."

# Every `typescript@<version>:` snapshot key declared in the lockfile.
#
# The version is matched as "everything up to the colon" rather than as plain
# `X.Y.Z`. A stricter pattern silently skips the keys that matter most —
# prereleases (`7.0.0-rc`), patched entries (`5.9.2(patch_hash=...)`) and peer
# suffixes — which would make this guard quietly pass on exactly the
# regressions it exists to catch.
installed=$(
  grep -oE '^  typescript@[^:]+:' "$lockfile" |
    sed -E 's/^  typescript@//; s/:$//' |
    sort -u || true
)

if [ -z "$installed" ]; then
  echo "error: found no TypeScript entries in ${lockfile} — is the lockfile complete?"
  exit 1
fi

# Compare on the bare version, but report the raw key, so a patched or
# peer-suffixed copy of the approved version is still identified precisely.
allowed=$(printf '%s\n%s\n' "$EXPECTED_VERSION" "$ALLOWED_EXTRA_VERSIONS")

unexpected=$(
  echo "$installed" |
    grep -vxF -f <(echo "$allowed") || true
)

if [ -n "$unexpected" ]; then
  echo ""
  echo "error: unexpected TypeScript version(s) in the installed graph:"
  formatted="  - ${unexpected//$'\n'/$'\n  - '}"
  echo "$formatted"
  echo ""
  echo "Only ${EXPECTED_VERSION} is approved (plus the documented"
  echo "ALLOWED_EXTRA_VERSIONS in this script). Find the consumer with:"
  echo "  pnpm why typescript -r"
  echo ""
  echo "A version below 7 almost always means some tool still depends on the"
  echo "removed JavaScript TypeScript compiler API. Replace that tool rather"
  echo "than pinning an older TypeScript alongside it."
  exit 1
fi

echo "Installed TypeScript graph is within policy:"
formatted_installed="  - ${installed//$'\n'/$'\n  - '}"
echo "$formatted_installed"
exit 0
