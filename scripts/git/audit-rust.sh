#!/usr/bin/env sh

# Supply-chain checks over the crate graph: `cargo deny` for licences, bans and
# sources, `cargo audit` for RustSec advisories against Cargo.lock. They overlap
# on advisories and disagree usefully -- cargo-deny walks the feature-resolved
# graph, cargo-audit reads the lockfile wholesale -- so both run.
#
# Policy lives in `deny.toml`, which is green today; see its header.
#
# Neither tool is a workspace dependency, so a missing one prints how to get it
# and does not fail. This runs behind `STYLEX_SLOW=1` on pre-push, where hard
# failing on a tool the developer never installed would just teach them to stop
# opting in. A tool that *is* installed and reports something still fails.

set -u

status=0
missing=''

run_audit() {
  tool=$1
  shift

  if ! command -v "cargo-$tool" >/dev/null 2>&1; then
    missing="${missing} cargo-${tool}"
    return 0
  fi

  echo "--- cargo $tool ---"
  # Deliberately not short-circuited: the two tools cover different ground, so a
  # cargo-deny failure is no reason to skip the advisory scan.
  if ! cargo "$tool" "$@"; then
    status=1
  fi
}

run_audit deny check
run_audit audit

if [ -n "$missing" ]; then
  echo
  echo "Skipped, not installed:${missing}"
  echo "  cargo install${missing} --locked"
fi

exit "$status"
