# shellcheck shell=bash

# Shared functions for the per-crate Rust scripts `index.sh`, `coverage.sh` and
# `flamegraph.sh`.
#
# The three scripts each held their own copy of the marker list, the directory
# search and the target-directory name. The copies then diverged: a correction
# went into one and not into the other two. Keep the code here, so that one
# correction changes all three.

# The markers that show that a crate holds a Rust test.
CRATE_TEST_MARKERS="#\[test\]|test_transform\(|test!\("

# The same markers, and also the module gate. Only the test runner reads the
# gate: a gate with no test in it gives a coverage tool nothing to measure.
# shellcheck disable=SC2034  # index.sh reads this, and shellcheck cannot see it from here.
CRATE_TEST_MARKERS_WITH_GATE="${CRATE_TEST_MARKERS}|#\[cfg\(test\)\]"

# Gives a true status if the crate holds one or more Rust tests.
#
# The caller supplies the markers, because the scripts do not all read the same
# set.
#
# Only the directories that exist go to grep. POSIX and GNU tell grep to give
# exit status 0 for a match with `-q`, even after an error, and BSD grep and
# GNU grep both obey this. A grep that does not obey it reads a missing
# directory as "no test found". The scripts must not depend on the difference.
crate_has_tests() {
  local patterns="$1"
  local directories=()
  local directory

  for directory in src tests; do
    # Ask for a directory and not for a name. A crate that holds a file with
    # this name must not give grep a path that grep refuses.
    if [ -d "$directory" ]; then
      directories+=("$directory")
    fi
  done

  # Stop before grep runs if the crate has neither directory. This guard is
  # necessary: bash 3.2 stops with an error for an empty array when the script
  # sets `-u`, which `coverage.sh` does.
  if [ ${#directories[@]} -eq 0 ]; then
    return 1
  fi

  grep -qRE --include="*.rs" "$patterns" "${directories[@]}"
}

# Prints the name of the crate directory. Changes each character that a path
# must not hold into an underscore.
#
# The name comes from an expansion, and not from a pipe out of `basename`.
# `basename` adds a newline, and `tr` changes that newline into an underscore.
# Every coverage directory thus held one more underscore than the crate name.
crate_slug() {
  printf '%s' "${PWD##*/}" | tr -c '[:alnum:]_-' '_'
}

# Prints the absolute path of the workspace root.
#
# The path starts at this file and not at the calling script, because the
# scripts are at a different depth from this library. `cd -P` follows a
# symbolic link to the true directory, because a logical path removes `..`
# as text and can then leave the repository.
crate_workspace_root() {
  local library_directory

  library_directory="$(cd -P "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

  (cd -P "$library_directory/../../../.." && pwd -P)
}
