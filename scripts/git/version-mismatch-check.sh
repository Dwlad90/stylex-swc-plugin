#!/bin/bash

# The manifest gate, run by the lefthook `version-mismatch` pre-commit job and
# in CI by the `pr-validation` matrix and the docs-validation format job. Two
# assertions, deliberately in one script so that every call site picks up
# either of them with no change to its wiring.
#
#   1. `syncpack lint`  -- formatting, field ordering and manifest shape. It no
#      longer asserts anything about *versions*; see the `catalogs` comment in
#      pnpm-workspace.yaml for why those groups are ignored.
#   2. `catalog-integrity.mjs manifests` -- the assertion syncpack gave up:
#      every dependency version is declared once, by name, in the catalogs.
#
# Both run on every invocation rather than short-circuiting on the first
# failure. A manifest that is unformatted is usually also the manifest that
# reintroduced a literal range, and fixing one at a time costs a commit cycle
# per problem.

set -uo pipefail

status=0

echo "Checking dependency versions across workspace..."

# Addressed by path, not `pnpm exec` -- see the `Gotchas` section of
# guidelines/git/HOOKS.md. Both callers, the hook and CI, run from the
# repository root.
if ! ./node_modules/.bin/syncpack lint; then
    echo ""
    echo "Please run 'pnpm syncpack fix' to fix the mismatches or fix them manually."
    status=1
fi

if ! node ./scripts/git/catalog-integrity.mjs manifests; then
    status=1
fi

if [ "$status" -eq 0 ]; then
    echo "All dependencies are in sync across the workspace."
fi

exit "$status"
