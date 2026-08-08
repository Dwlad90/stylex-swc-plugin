#!/usr/bin/env sh

# Refreshes the resolved-config golden that `lefthook-config.test.mjs` compares
# against. Run from the `hooks:dump` npm script, after reviewing the diff the
# test printed.
#
# A script rather than an inline redirect because of `lefthook-local.yml`.
# `lefthook dump` always merges it and offers no way to opt out -- the file is
# keyed off the config's own name, so neither `LEFTHOOK_CONFIG` nor a `dump`
# flag excludes it. A developer using that sanctioned override would therefore
# write their personal skips into a snapshot every other developer and CI is
# held to, and the diff reads as an intentional weakening of the shared hooks.
# Refusing is the only outcome that cannot silently do that.
#
# The test skips the golden comparison under the same condition, so nothing
# routinely sends a developer here while the file exists.

set -e

if [ -f lefthook-local.yml ]; then
  echo "Refusing to refresh the golden: lefthook-local.yml is present." >&2
  echo "\`lefthook dump\` merges it, so the snapshot would capture your personal" >&2
  echo "overrides. Move it aside, re-run, then put it back." >&2
  exit 1
fi

# Redirected only after lefthook exits 0. `>` truncates before the command
# runs, so a failed dump written in place would leave an empty golden -- and an
# empty golden is a diff a reviewer might wave through.
GOLDEN=scripts/git/__snapshots__/lefthook-dump.yml
DUMPED=$(./node_modules/.bin/lefthook dump)

printf '%s\n' "$DUMPED" > "$GOLDEN"
echo "Wrote $GOLDEN."
