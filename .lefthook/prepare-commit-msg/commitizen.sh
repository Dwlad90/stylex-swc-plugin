#!/usr/bin/env sh

# Interactive commit message prompt. Invoked by lefthook's `prepare-commit-msg`
# hook with `interactive: true`, which supplies /dev/tty as stdin -- the shell
# redirect the husky version needed is now the runner's job.
#
# Lefthook passes the full git hook argv through to scripts, so the guard below
# reads the same positional arguments the husky hook did.

# COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2
SHA=$3

# Amending: git passes the existing commit's SHA as $3, and its message is
# already there to edit.
if [ -n "$SHA" ]; then
  exit 0
fi

# The message already came from somewhere: -m/-F (`message`), a merge, or a
# squash. Every other source falls through to the prompt -- including no source
# at all, which is the plain `git commit` case.
case "$COMMIT_SOURCE" in
  message | merge | squash) exit 0 ;;
esac

# Deliberately not `pnpm exec`: one less process sitting between the terminal
# and the prompt. Lefthook runs jobs from the repository root, so the relative
# path resolves.
exec node_modules/.bin/cz --hook
