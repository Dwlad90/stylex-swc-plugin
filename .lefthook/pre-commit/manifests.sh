#!/usr/bin/env sh

# Formats the staged package.json manifests passed as arguments.
#
# Runs from the repository root: lefthook hands over repo-relative staged paths,
# so `.syncpackrc` -- and the `./node_modules/.bin` invocations below -- resolve
# the same way the arguments do. On why the binaries are addressed by path
# rather than through `pnpm exec`, see guidelines/git/HOOKS.md.
#
# Both tools run here, in this order, so that they are sequenced against each
# other. Two concurrent writers to one manifest is the race this file exists to
# prevent -- which is also why `lefthook.yml` excludes manifests from the
# `data-files` job rather than letting them match both.

set -e

if [ "$#" -eq 0 ]; then
  exit 0
fi

# syncpack takes one `--source` per path, so the arguments have to be
# interleaved rather than appended. Appending them is what made syncpack reject
# the second manifest, failing this hook on exactly the commits that touch more
# than one.
#
# The rewrite happens in a subshell so that `$@` still holds the plain paths
# afterwards, and goes through the positional parameters rather than a string so
# that a path containing spaces survives.
(
  paths=$#
  for manifest in "$@"; do
    set -- "$@" --source "$manifest"
  done
  shift "$paths"

  exec ./node_modules/.bin/syncpack format --config .syncpackrc "$@"
)

./node_modules/.bin/oxfmt --no-error-on-unmatched-pattern "$@"
