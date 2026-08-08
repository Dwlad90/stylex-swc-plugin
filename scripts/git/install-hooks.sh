#!/usr/bin/env sh

# Installs lefthook git hooks. Run from the `prepare` npm script.
#
# The defensive unset is the whole reason this wrapper exists. husky set
# `core.hooksPath` to `.husky/_`, and lefthook refuses to install while that is
# set: it prints advice and exits 0. On an existing clone that means
# `pnpm install` succeeds, reports nothing unusual, and leaves the repository
# with no hooks at all. The unset is narrowly scoped to husky's exact value so a
# developer's intentional custom hooks path is never clobbered.
#
# Note this only needs to happen once per clone, not per worktree: lefthook
# writes into the common git dir, so a single install covers every worktree.

set -e

# CI checks out fresh, never commits, and runs its own named jobs for
# everything the hooks would do. Installing there would only mutate the
# runner's git config for no benefit.
# `-n` rather than `= true`: CI providers agree on setting CI, not on its value.
if [ -n "$CI" ] || [ "$LEFTHOOK" = "0" ]; then
  echo "Skipping lefthook install."
  exit 0
fi

HOOKS_PATH=$(git config --get core.hooksPath || true)

if [ "$HOOKS_PATH" = ".husky/_" ]; then
  echo "Removing husky's core.hooksPath so lefthook can install..."
  git config --unset core.hooksPath
fi

# Addressed by path like every other npm binary in the hooks -- see the
# `Gotchas` section of guidelines/git/HOOKS.md. npm runs `prepare` from the
# package root, so the relative path resolves, and it pins the install to the
# workspace's own lefthook rather than to whatever a stale global provides.
#
# Never `--force`: that writes lefthook's hooks into husky's `.husky/_`,
# resurrecting the directory this migration deletes.
exec ./node_modules/.bin/lefthook install
