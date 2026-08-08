#!/usr/bin/env bash

# Rejects leftover merge-conflict markers.
#
#   no-merge-conflicts.sh staged   pre-commit: unmerged index entries plus
#                                  markers in staged content (the default)
#   no-merge-conflicts.sh pushed   pre-push: markers in the exact ref updates
#                                  git supplies on stdin
#
# `git diff --check` does the detection, so this costs one git call in the
# common case and needs no marker patterns of its own.

set -u

mode="${1:-staged}"

# `git diff --check` reports one line per offending line; reduce it to paths.
conflict_paths() {
  sed -nE 's/^(.+):[0-9]+: leftover conflict marker$/\1/p'
}

# `git diff --check` exits non-zero both for the diagnostics we are looking for
# and for operational failures. Passing a bad ref would otherwise look exactly
# like a clean tree, so anything git reports as fatal has to fail closed rather
# than be read as "no conflicts".
checked_diff() {
  local output status
  output=$(LC_ALL=C git diff --check "$@" 2>&1)
  status=$?

  if [ "$status" -ne 0 ] && printf '%s\n' "$output" | grep -Eq '^(fatal|error):'; then
    printf '%s\n' "$output" >&2
    return 2
  fi

  printf '%s\n' "$output"
}

case "$mode" in
  staged)
    # Two distinct failures: an index that still has unmerged entries, and
    # markers that were resolved into the staged content by hand.
    unmerged=$(git diff --cached --name-only --diff-filter=U) || exit 2
    diff_output=$(checked_diff --cached) || exit $?

    conflicts=$(
      {
        printf '%s\n' "$unmerged"
        printf '%s\n' "$diff_output" | conflict_paths
      } | sed '/^$/d' | sort -u
    )
    origin_description="in the following staged files"
    ;;

  pushed)
    found=''

    # git feeds pre-push one `<local ref> <local oid> <remote ref> <remote oid>`
    # line per ref being updated.
    while read -r _local_ref local_oid _remote_ref remote_oid; do
      # An all-zero local oid deletes the remote ref; it introduces no content.
      if [[ "$local_oid" =~ ^0+$ ]]; then
        continue
      fi

      # Resolve the type before the commit, so that "object is not in the
      # database" is distinguishable from "object is not a commit". Collapsing
      # the two is a fail-open: an oid git cannot find would be skipped as
      # though it were a tag, and the push would sail through unchecked.
      object_type=$(git cat-file -t "$local_oid" 2>/dev/null) || {
        echo "Unable to inspect local object $local_oid" >&2
        exit 2
      }

      local_commit=$(git rev-parse --verify --quiet "${local_oid}^{commit}") || local_commit=''

      if [ -z "$local_commit" ]; then
        if [ "$object_type" = 'commit' ]; then
          echo "Unable to inspect local commit $local_oid" >&2
          exit 2
        fi
        # A tag pointing at a tree or blob has no tree to diff. Skipping it is
        # correct; erroring would break `git push --tags`.
        continue
      fi

      if [[ "$remote_oid" =~ ^0+$ ]]; then
        # A branch the remote has never seen has no base to diff against.
        # Bound the scan to what is not already on the integration branch
        # rather than walking all of history; fall back to the empty tree when
        # there is no such branch (a fresh clone with no `origin/develop`).
        base=$(git merge-base "$local_commit" origin/develop 2>/dev/null) || base=''
        if [ -z "$base" ]; then
          base=$(git hash-object -t tree --stdin </dev/null)
        fi
      else
        base=$(git rev-parse --verify --quiet "${remote_oid}^{commit}") || base=''
        if [ -z "$base" ]; then
          echo "Unable to inspect remote object $remote_oid" >&2
          exit 2
        fi
      fi

      diff_output=$(checked_diff "$base" "$local_commit") || exit $?
      paths=$(printf '%s\n' "$diff_output" | conflict_paths)

      if [ -n "$paths" ]; then
        found="${found}${paths}"$'\n'
      fi
    done

    conflicts=$(printf '%s' "$found" | sed '/^$/d' | sort -u)
    origin_description="in the commits being pushed"
    ;;

  *)
    echo "Unknown mode: \"$mode\" (expected 'staged' or 'pushed')" >&2
    exit 2
    ;;
esac

if [ -n "$conflicts" ]; then
  echo
  echo "There are unresolved merge conflicts ${origin_description}:"
  printf '%s\n' "$conflicts"
  exit 1
fi
