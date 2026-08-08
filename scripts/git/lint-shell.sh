#!/usr/bin/env sh

# Shellchecks every tracked `*.sh`. The CI counterpart of the pre-commit `shell`
# job, which lints only what you staged.
#
# A script rather than an inline `$(git ls-files '*.sh')` in `package.json`:
# command substitution word-splits, so a path containing a space would arrive as
# two arguments, and an empty result would leave shellcheck with no files and a
# usage error. NUL-delimited plus an explicit empty check avoids both.

set -e

if [ -z "$(git ls-files '*.sh')" ]; then
  exit 0
fi

git ls-files -z '*.sh' | xargs -0 ./node_modules/.bin/shellcheck -x
