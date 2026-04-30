#!/usr/bin/env bash
# scripts/deps-prune.sh
#
# Safely prune unused dependencies across the polyglot monorepo.
#
# Tools:
#   - knip           (Node.js / pnpm workspaces) -- repo devDependency
#   - cargo-machete  (Rust workspaces)           -- `cargo install cargo-machete --locked`
#
# SAFETY MODEL
#   * Default mode is DRY-RUN. Nothing is written. Use --apply to mutate manifests.
#   * --apply takes a snapshot of manifest files BEFORE touching them and
#     verifies the workspace still compiles / typechecks AFTER. On any failure
#     the manifest changes are auto-reverted from the snapshot.
#   * For Node, --apply only removes entries from package.json
#     (`--include dependencies`). It will NEVER touch source files, exports or
#     types — knip's other auto-fixes are explicitly disabled.
#   * Known false positives must be declared once and respected forever:
#       Rust:  [package.metadata.cargo-machete] ignored = ["foo"]
#       Node:  knip.json -> "ignoreDependencies"
#
# USAGE
#   scripts/deps-prune.sh                # dry-run, both ecosystems  (CI-safe)
#   scripts/deps-prune.sh --apply        # actually remove unused deps + verify
#   scripts/deps-prune.sh --node         # restrict to Node.js side
#   scripts/deps-prune.sh --rust         # restrict to Rust side
#   scripts/deps-prune.sh --apply --with-tests  # also run pnpm test in verify
#                                                (slow, but catches string-ref
#                                                 deps used only at test time)
#   scripts/deps-prune.sh --apply --no-verify   # skip the safety verify step
#                                                 (NOT recommended)
#
# EXIT CODES
#   0  no unused deps found / all changes verified clean
#   1  unused deps found in dry-run mode (CI signal)
#   2  apply succeeded BUT verification failed -> all changes auto-reverted
#   3  bad usage / missing tools

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

MODE="dry-run"
RUN_NODE=1
RUN_RUST=1
VERIFY=1
WITH_TESTS=0

for arg in "$@"; do
  case "$arg" in
    --apply)       MODE="apply" ;;
    --dry-run)     MODE="dry-run" ;;
    --node)        RUN_RUST=0 ;;
    --rust)        RUN_NODE=0 ;;
    --no-verify)   VERIFY=0 ;;
    --with-tests)  WITH_TESTS=1 ;;
    -h|--help)     sed -n '2,33p' "$0"; exit 0 ;;
    *) echo "Unknown arg: $arg" >&2; exit 3 ;;
  esac
done

c_bold()    { printf '\033[1m%s\033[0m\n' "$*"; }
c_section() { printf '\n\033[1;36m▶ %s\033[0m\n' "$*"; }
c_ok()      { printf '\033[1;32m✓ %s\033[0m\n' "$*"; }
c_warn()    { printf '\033[1;33m! %s\033[0m\n' "$*"; }
c_err()     { printf '\033[1;31m✗ %s\033[0m\n' "$*" >&2; }

require() {
  command -v "$1" >/dev/null 2>&1 || { c_err "required tool not found: $1"; exit 3; }
}

# Snapshot manifest files matching the given glob into a temp dir.
snapshot_manifests() {
  local glob="$1"
  local dir; dir="$(mktemp -d)"
  while IFS= read -r -d '' f; do
    mkdir -p "$dir/$(dirname "$f")"
    cp "$f" "$dir/$f"
  done < <(git ls-files -z -- "$glob")
  echo "$dir"
}

restore_manifests() {
  local dir="$1"; local glob="$2"
  while IFS= read -r -d '' f; do
    [[ -f "$dir/$f" ]] && cp "$dir/$f" "$f"
  done < <(git ls-files -z -- "$glob")
}

OVERALL_STATUS=0

###############################################################################
# Node.js / pnpm
###############################################################################
if [[ $RUN_NODE -eq 1 ]]; then
  c_section "Node.js workspaces — knip [$MODE]"
  require pnpm

  # `--include dependencies` restricts knip to package.json mutations only.
  # All other knip categories (files, exports, types, ...) are ignored here so
  # `--fix` can NEVER touch source code.
  # We use --no-exit-code and detect the "Unused (dev)Dependencies" sections
  # ourselves: knip exits non-zero on harmless plugin warnings (e.g.
  # unparseable user configs) which would otherwise cause false alarms in CI.
  KNIP_BASE=(--no-progress --reporter symbols --include dependencies --no-config-hints --no-exit-code)

  knip_has_findings() {
    grep -qE '^Unused (dev|optional|peer)?[Dd]ependencies' "$1"
  }

  if [[ "$MODE" == "dry-run" ]]; then
    OUT="$(mktemp)"
    pnpm exec knip "${KNIP_BASE[@]}" | tee "$OUT"
    if knip_has_findings "$OUT"; then
      c_warn "knip: unused dependencies found (dry-run; nothing changed)"
      OVERALL_STATUS=1
    else
      c_ok "knip: no unused Node.js dependencies"
    fi
    rm -f "$OUT"
  else
    SNAP="$(snapshot_manifests '*package.json')"
    c_bold "snapshot: $SNAP"
    pnpm exec knip "${KNIP_BASE[@]}" --fix || true

    if [[ $VERIFY -eq 1 ]]; then
      desc="install + typecheck + build"
      [[ $WITH_TESTS -eq 1 ]] && desc="$desc + test"
      c_section "Verifying Node.js workspace ($desc)"
      verify_ok=1
      pnpm install --prefer-offline >/dev/null 2>&1 || verify_ok=0
      [[ $verify_ok -eq 1 ]] && { pnpm typecheck >/dev/null 2>&1 || verify_ok=0; }
      [[ $verify_ok -eq 1 ]] && { pnpm build >/dev/null 2>&1 || verify_ok=0; }
      if [[ $verify_ok -eq 1 && $WITH_TESTS -eq 1 ]]; then
        pnpm test >/dev/null 2>&1 || verify_ok=0
      fi
      if [[ $verify_ok -eq 1 ]]; then
        c_ok "Node.js verification passed"
        rm -rf "$SNAP"
      else
        c_err "Node.js verification failed — reverting package.json changes"
        c_warn "TIP: declare false positives in knip.json -> ignoreDependencies"
        restore_manifests "$SNAP" '*package.json'
        pnpm install --prefer-offline >/dev/null 2>&1 || true
        rm -rf "$SNAP"
        OVERALL_STATUS=2
      fi
    else
      c_warn "Skipping verification (--no-verify); snapshot kept at $SNAP"
    fi
  fi
fi

###############################################################################
# Rust
###############################################################################
if [[ $RUN_RUST -eq 1 ]]; then
  c_section "Rust workspaces — cargo-machete [$MODE]"
  require cargo-machete

  if [[ "$MODE" == "dry-run" ]]; then
    if cargo machete --with-metadata . ; then
      c_ok "cargo-machete: no unused Rust dependencies"
    else
      c_warn "cargo-machete: unused dependencies found (dry-run; nothing changed)"
      [[ $OVERALL_STATUS -lt 1 ]] && OVERALL_STATUS=1
    fi
  else
    SNAP="$(snapshot_manifests '*Cargo.toml')"
    c_bold "snapshot: $SNAP"
    cargo machete --fix --with-metadata . || true

    if [[ $VERIFY -eq 1 ]]; then
      desc="cargo check --tests --benches"
      [[ $WITH_TESTS -eq 1 ]] && desc="$desc + cargo test"
      c_section "Verifying Rust workspace ($desc)"
      verify_ok=1
      cargo check --workspace --all-features --tests --benches >/dev/null 2>&1 \
        || verify_ok=0
      if [[ $verify_ok -eq 1 && $WITH_TESTS -eq 1 ]]; then
        cargo test --workspace --all-features --no-fail-fast >/dev/null 2>&1 \
          || verify_ok=0
      fi
      if [[ $verify_ok -eq 1 ]]; then
        c_ok "Rust verification passed"
        rm -rf "$SNAP"
      else
        c_err "Rust verification failed — reverting Cargo.toml changes"
        c_warn "Declare false positives in the offending crate's Cargo.toml:"
        c_warn "    [package.metadata.cargo-machete]"
        c_warn "    ignored = [\"<crate>\"]"
        restore_manifests "$SNAP" '*Cargo.toml'
        rm -rf "$SNAP"
        OVERALL_STATUS=2
      fi
    else
      c_warn "Skipping verification (--no-verify); snapshot kept at $SNAP"
    fi
  fi
fi

###############################################################################
# Summary
###############################################################################
echo
case $OVERALL_STATUS in
  0) c_ok    "Done." ;;
  1) c_warn  "Done with findings (dry-run)." ;;
  2) c_err   "Apply failed verification; manifests reverted." ;;
esac
exit "$OVERALL_STATUS"
