#!/usr/bin/env bash
# scripts/coverage-missing.sh
#
# Pinpoint uncovered code in the Rust workspace using cargo-llvm-cov.
#
# This is the diagnostic companion to `pnpm test:coverage:workspace`, which only
# reports pass/fail percentages per file. This script additionally prints the
# exact `file:line:col` list of code that no test exercises, so you know
# precisely which branches still need a test.
#
# WHY NOT JUST `--show-missing-lines`?
#   cargo-llvm-cov's `--show-missing-lines` only lists uncovered *lines*, never
#   uncovered *regions* (sub-line segments such as a match arm, the `?`-on-None
#   path, or a closure branch). Region coverage is exactly what CI gates on, so
#   a build could fail with "1 uncovered region" yet print no location at all.
#   This script instead parses the JSON export and reports every uncovered
#   region with line:col coordinates — lines and sub-line regions alike.
#
# THREE robustness fixes over a naive `--show-missing-lines` gate:
#   1. Regions, not just lines — sub-line gaps are reported with line:col.
#   2. Monomorphization merge — regions are summed across generic instantiations
#      by source coordinate. A generic fn instantiated by a type that bails out
#      early (often a test mock) leaves per-instantiation gaps that llvm-cov
#      counts as uncovered even though the source line *is* exercised by another
#      instantiation. A region is reported only when no instantiation runs it;
#      the delta from llvm-cov's raw count is explained in a note.
#   3. Scope filtering — cargo-llvm-cov's target dir is stateful: a `-p <crate>`
#      run can fold in leftover instrumented object files from an earlier
#      full-workspace run (e.g. dependency crates), producing a noisy,
#      non-deterministic file list. The report is filtered to the requested
#      scope so the output is stable no matter what the target dir holds.
#
# REQUIREMENTS
#   - Rust nightly toolchain          : rustup toolchain install nightly
#   - cargo-llvm-cov + cargo-nextest  : cargo install cargo-llvm-cov cargo-nextest --locked
#   - python3 (for region detail)     : falls back to line-only output if absent
#
# USAGE
#   scripts/coverage-missing.sh                 # whole workspace (matches CI excludes)
#   scripts/coverage-missing.sh stylex_css      # single crate (fast iteration)
#   scripts/coverage-missing.sh -p stylex_css   # same, explicit flag
#   scripts/coverage-missing.sh --show-phantoms # also print generic per-instantiation gaps
#   scripts/coverage-missing.sh --strict-phantoms # fail on generic per-instantiation gaps too
#   scripts/coverage-missing.sh --html          # also write an HTML report (second run)
#   scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
#   scripts/coverage-missing.sh -h | --help
#
# EXIT STATUS
#   0  every measured source region is exercised by at least one test
#   1  one or more source regions are unexercised (the `file:line:col` list is
#      printed above), or --strict-phantoms found uncovered generic instantiations

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Crates excluded from workspace coverage, kept in sync with the
# `test:coverage:workspace` script in the root package.json.
EXCLUDED_CRATES=(
  stylex_logs
  stylex_compiler_rs
  stylex_test_parser
  stylex_css_parser
  stylex_transform
)
WORKSPACE_EXCLUDES=()
for crate in "${EXCLUDED_CRATES[@]}"; do
  WORKSPACE_EXCLUDES+=(--exclude "$crate")
done

# Generated test/bench/example files are never counted, matching CI.
IGNORE_REGEX='(tests?|benches?|examples)/'

usage() {
  cat <<'EOF'
coverage-missing.sh — pinpoint uncovered code in the Rust workspace.

USAGE
  scripts/coverage-missing.sh                 # whole workspace (matches CI excludes)
  scripts/coverage-missing.sh stylex_css      # single crate (fast iteration)
  scripts/coverage-missing.sh -p stylex_css   # same, explicit flag
  scripts/coverage-missing.sh --show-phantoms # also print generic per-instantiation gaps
  scripts/coverage-missing.sh --strict-phantoms # fail on generic per-instantiation gaps too
  scripts/coverage-missing.sh --html          # also write an HTML report (second run)
  scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
  scripts/coverage-missing.sh -h | --help

EXIT STATUS
  0  every measured source region is exercised by at least one test
  1  one or more source regions are unexercised (the file:line:col list is printed above),
     or --strict-phantoms found uncovered generic instantiations
EOF
  exit "${1:-0}"
}

package=""
html=0
open=0
show_phantoms=0
strict_phantoms=0

while [ $# -gt 0 ]; do
  case "$1" in
    -h | --help) usage 0 ;;
    --show-phantoms) show_phantoms=1 ;;
    --strict-phantoms)
      show_phantoms=1
      strict_phantoms=1
      ;;
    --html) html=1 ;;
    --open)
      html=1
      open=1
      ;;
    -p | --package)
      [ $# -ge 2 ] || { echo "error: $1 requires a crate name" >&2; exit 2; }
      package="$2"
      shift
      ;;
    -*) echo "error: unknown option '$1' (try --help)" >&2; exit 2 ;;
    *)
      if [ -n "$package" ]; then
        echo "error: unexpected argument '$1'" >&2
        exit 2
      fi
      package="$1"
      ;;
  esac
  shift
done

# Select scope: a single crate (fast) or the whole workspace (CI parity).
# `scope_*` is handed to the Python reporter so its file list matches exactly the
# crate(s) the run targeted, regardless of stale objects in the target dir.
# Crate directories use hyphens where package names use underscores.
scope=()
if [ -n "$package" ]; then
  scope=(-p "$package")
  scope_mode="crate"
  scope_value="${package//_/-}"
  echo "==> Coverage for crate: $package"
else
  scope=(--workspace "${WORKSPACE_EXCLUDES[@]}")
  scope_mode="workspace"
  scope_value=""
  for crate in "${EXCLUDED_CRATES[@]}"; do
    scope_value+="${crate//_/-},"
  done
  echo "==> Coverage for workspace (excluding non-instrumented crates)"
fi

tmp_json="$(mktemp "${TMPDIR:-/tmp}/coverage-missing.XXXXXX")"
tmp_log="$(mktemp "${TMPDIR:-/tmp}/coverage-missing.XXXXXX")"
cleanup() { rm -f "$tmp_json" "$tmp_log"; }
trap cleanup EXIT

# Single instrumented run. The JSON export is the source of truth for both the
# summary table and the precise miss list rendered below. (The `report`
# subcommand is intentionally NOT used: it globs every object file in the target
# dir and would fold in stale results from an earlier differently-scoped run.)
run_coverage() {
  # `tee` mirrors progress to the terminal while capturing it for stale-artifact
  # detection. PIPESTATUS[0] is cargo's real exit status (tee always succeeds).
  cargo +nightly llvm-cov nextest "${scope[@]}" \
    --all-features \
    --ignore-filename-regex "$IGNORE_REGEX" \
    --json --output-path "$tmp_json" 2>&1 | tee "$tmp_log"
  return "${PIPESTATUS[0]}"
}

set +e
run_coverage
status=$?
set -e

# cargo-llvm-cov's target dir is stateful: a scope change can leave its object
# manifest pointing at .dylib/.rlib files that a later build removed, so the
# export aborts with "could not load coverage". Self-heal with a one-time clean
# + retry rather than forcing every run through a slow full rebuild.
if [ "$status" -ne 0 ] \
  && grep -qiE "could not load coverage|failed to load coverage|no such file" "$tmp_log"; then
  echo "==> Stale coverage artifacts detected; running 'cargo llvm-cov clean' and retrying once..." >&2
  cargo llvm-cov clean --workspace
  set +e
  run_coverage
  status=$?
  set -e
fi

if [ "$status" -ne 0 ]; then
  echo "error: coverage run failed (see output above)" >&2
  exit "$status"
fi

# Optional HTML report. `--html` cannot share an invocation with `--json`, so it
# costs a second instrumented run; it is opt-in and rare.
if [ "$html" -eq 1 ]; then
  html_flags=(--html)
  [ "$open" -eq 1 ] && html_flags+=(--open)
  cargo +nightly llvm-cov nextest "${scope[@]}" \
    --all-features \
    --ignore-filename-regex "$IGNORE_REGEX" \
    "${html_flags[@]}"
fi

# Render the summary table and the exact uncovered locations, and set the exit
# status, all from the JSON export.
if command -v python3 >/dev/null 2>&1; then
  python3 - "$tmp_json" "$REPO_ROOT" "$IGNORE_REGEX" "$scope_mode" "$scope_value" "$show_phantoms" "$strict_phantoms" <<'PY'
import json
import os
import re
import sys
from collections import defaultdict

json_path, repo_root, ignore_regex, scope_mode, scope_value, show_phantoms_arg, strict_phantoms_arg = sys.argv[1:8]
ignore = re.compile(ignore_regex)
show_phantoms_enabled = show_phantoms_arg == "1"
strict_phantoms_enabled = strict_phantoms_arg == "1"

with open(json_path) as fh:
    report = json.load(fh)

export = report["data"][0]

# Restrict the report to the files the run actually targeted, so stale objects
# from an earlier differently-scoped run never leak in.
if scope_mode == "crate":
    needle = f"/{scope_value}/"

    def in_scope(filename):
        return needle in filename
else:
    excluded = [f"/{name}/" for name in scope_value.split(",") if name]

    def in_scope(filename):
        return not any(frag in filename for frag in excluded)


def keep(filename):
    if not filename or ignore.search(filename):
        return False
    # Defensive: never report on toolchain/registry sources.
    if "/rustc/" in filename or "/.cargo/" in filename or "/.rustup/" in filename:
        return False
    return in_scope(filename)


def rel(path):
    try:
        return os.path.relpath(path, repo_root)
    except ValueError:
        return path


# ── Summary table (derived from the same export as the miss list) ────────────
# `export["files"]` is the authoritative measured set: cargo-llvm-cov has
# already applied its full ignore regex (our pattern PLUS its built-in
# `tests.rs`/`*_tests.rs`/target/registry defaults) to it. `export["functions"]`
# is NOT filtered, so the region scan below is restricted to this set to inherit
# exactly the same exclusions CI uses.
file_entries = [f for f in export.get("files", []) if keep(f.get("filename", ""))]
measured = {f["filename"] for f in file_entries}
rows = sorted((rel(f["filename"]), f["summary"]) for f in file_entries)
name_width = max([len("File")] + [len(name) for name, _ in rows])
name_width = min(name_width, 70)


def pct(covered, count):
    return 100.0 if count == 0 else covered / count * 100.0


agg = {"regions": [0, 0], "functions": [0, 0], "lines": [0, 0]}

print()
print(f"{'File':<{name_width}}  {'Regions':>9}  {'Functions':>9}  {'Lines':>9}")
print("-" * (name_width + 33))
for name, summary in rows:
    cells = []
    for kind in ("regions", "functions", "lines"):
        covered = summary.get(kind, {}).get("covered", 0)
        count = summary.get(kind, {}).get("count", 0)
        agg[kind][0] += covered
        agg[kind][1] += count
        cells.append(f"{pct(covered, count):>8.2f}%")
    print(f"{name:<{name_width}}  " + "  ".join(cells))
print("-" * (name_width + 33))
total_cells = "  ".join(f"{pct(*agg[kind]):>8.2f}%" for kind in ("regions", "functions", "lines"))
print(f"{'TOTAL':<{name_width}}  " + total_cells)

# ── Uncovered regions, merged across monomorphizations ───────────────────────
# llvm-cov region kinds: 0 = Code (the kind region coverage is based on),
# 1 = Expansion, 2 = Skipped, 3 = Gap, 4 = Branch, 5/6 = MC/DC. Only "Code"
# regions are counted, matching `--fail-uncovered-regions`.
CODE_KIND = 0

# Sum execution counts per distinct source region across every instantiation. A
# region is genuinely unexercised only when that sum is zero.
#
# `region_zero_insts` additionally records, for each source region, the set of
# monomorphized instantiations (mangled function symbols) in which that region
# is *not* executed. When a region's total is > 0 but this set is non-empty, the
# region is a "phantom": covered in aggregate, yet left unexercised in some
# generic instantiation. Surfacing those locations is what lets a maintainer act
# on the per-instantiation gap (drive that instantiation through the branch, or
# collapse the redundant instantiations into one).
region_total = defaultdict(int)
region_zero_insts = defaultdict(set)
for function in export.get("functions", []):
    filenames = function.get("filenames", [])
    fn_name = function.get("name", "")
    for region in function.get("regions", []):
        # region = [line_start, col_start, line_end, col_end, count, file_id,
        #           expanded_file_id, kind]
        if len(region) < 8 or region[7] != CODE_KIND:
            continue
        file_id = region[5]
        filename = filenames[file_id] if file_id < len(filenames) else ""
        if filename not in measured:
            continue
        key = (filename, region[0], region[1], region[2], region[3])
        region_total[key] += region[4]
        if region[4] == 0:
            region_zero_insts[key].add(fn_name)

uncovered = sorted(key for key, count in region_total.items() if count == 0)

# Phantom regions: exercised in aggregate (total > 0) but missed by at least one
# monomorphization. These are exactly the instances llvm-cov tallies in
# `raw_notcovered` beyond the genuinely-unexercised `uncovered` set.
phantoms = sorted(
    (key, region_zero_insts[key])
    for key, count in region_total.items()
    if count > 0 and region_zero_insts.get(key)
)


_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_SNAKE = re.compile(r"[a-z][a-z0-9_]*")
_CAMEL = re.compile(r"[A-Z][a-z][A-Za-z0-9]*")
# Std/crate-root path noise we drop to keep the hint focused on our own code.
_DROP = {"core", "alloc", "std", "option", "result", "string", "stylex_css_parser"}


def readable_symbol(sym):
    """Best-effort human hint for a Rust v0 mangled symbol.

    We avoid a hard dependency on rustc-demangle: walk the v0 `<len><ident>`
    path components, keep the clean snake_case fns/modules and CamelCase types,
    and drop backref/lifetime/hash noise (`B19_`, `Cs<hash>`) plus std path
    roots — leaving e.g. `token_parser::TokenParser::SimpleToken::surrounded_by`."""
    comps = []
    i, n = 0, len(sym)
    while i < n:
        if sym[i].isdigit():
            j = i
            while j < n and sym[j].isdigit():
                j += 1
            length = int(sym[i:j])
            ident = sym[j : j + length]
            i = j + length
            if _IDENT.fullmatch(ident) and (_SNAKE.fullmatch(ident) or _CAMEL.fullmatch(ident)):
                if ident not in _DROP and ident not in comps:
                    comps.append(ident)
        else:
            i += 1
    return "::".join(comps[:8]) if comps else "<unknown>"

# llvm-cov's own (per-instantiation) uncovered-region tally, for the note below.
raw_notcovered = sum(
    f.get("summary", {}).get("regions", {}).get("notcovered", 0) for f in file_entries
)

if raw_notcovered > len(uncovered):
    suffix = ""
    if not show_phantoms_enabled:
        suffix = "\n      Re-run with --show-phantoms to print those generic instantiation gaps."
    print(
        f"\nnote: llvm-cov counts {raw_notcovered} uncovered region instance(s), but "
        f"{len(uncovered)} distinct source region(s) are truly unexercised.\n"
        "      The gap is generic monomorphization: a generic function instantiated by a "
        "type that\n"
        "      bails out early (often a test mock) leaves per-instantiation gaps that "
        "vanish once\n"
        f"      instantiations are merged.{suffix}"
    )


def print_phantoms():
    """Surface phantom regions: source code exercised in aggregate, yet missed by
    at least one monomorphization. These are the per-instantiation gaps behind
    llvm-cov's `notcovered` tally. We report them compactly — the distinct source
    lines per file, plus the functions whose instantiations leave gaps — so they
    are greppable without drowning the output in one entry per monomorphization."""
    if not phantoms:
        return
    lines_by_file = defaultdict(set)
    fns_by_file = defaultdict(set)
    for (filename, ls, cs, le, ce), insts in phantoms:
        lines_by_file[filename].add(ls)
        for sym in insts:
            hint = readable_symbol(sym)
            if hint != "<unknown>":
                fns_by_file[filename].add(hint)
    distinct = sum(len(v) for v in lines_by_file.values())
    print(
        "\nPhantom regions — source executed in the merged coverage (the crate is at\n"
        "100% line/region coverage once monomorphizations are combined), yet skipped by\n"
        "at least one generic instantiation. To drive llvm-cov's per-instantiation tally\n"
        "to zero, exercise the listed function for the missing type, or collapse the\n"
        "redundant instantiations into one. Source lines with gaps, per file:\n"
    )
    for filename in sorted(lines_by_file):
        lines = ", ".join(str(n) for n in sorted(lines_by_file[filename]))
        print(f"  {rel(filename)}: {lines}")
        for hint in sorted(fns_by_file[filename])[:12]:
            print(f"      fn: {hint}")
        extra = len(fns_by_file[filename]) - 12
        if extra > 0:
            print(f"      ... and {extra} more function(s)")
        print()
    print(f"{distinct} distinct source line(s) carry per-instantiation gaps.")


if raw_notcovered == 0:
    print("\n✓ No uncovered regions — every measured source region is exercised.")
    sys.exit(0)

if not uncovered:
    if show_phantoms_enabled:
        print_phantoms()
    if strict_phantoms_enabled and raw_notcovered > 0:
        print(
            f"\n✗ No distinct uncovered source regions, but llvm-cov reports "
            f"{raw_notcovered} uncovered generic instantiation gap(s)."
        )
        sys.exit(1)
    print(
        f"\n✓ No distinct uncovered source regions. llvm-cov reports {raw_notcovered} "
        "uncovered generic instantiation gap(s), treated as phantom by default."
    )
    sys.exit(0)

by_file = defaultdict(list)
for filename, line_start, col_start, line_end, col_end in uncovered:
    by_file[filename].append((line_start, col_start, line_end, col_end))

print("\nUncovered regions (not executed by any test):\n")
for filename in sorted(by_file):
    print(f"  {rel(filename)}")
    for line_start, col_start, line_end, col_end in sorted(by_file[filename]):
        if line_start == line_end:
            print(f"    line {line_start:<6} cols {col_start}-{col_end}")
        else:
            print(f"    lines {line_start}-{line_end:<6} cols {col_start}-{col_end}")
    print()

# Compact, grep-friendly `file: line, line, ...` list of the region start lines.
print("Uncovered lines (compact):")
for filename in sorted(by_file):
    start_lines = sorted({line_start for line_start, *_ in by_file[filename]})
    print(f"  {rel(filename)}: " + ", ".join(str(line) for line in start_lines))

region_count = sum(len(v) for v in by_file.values())
print(f"\n{region_count} uncovered region(s) across {len(by_file)} file(s).")
if show_phantoms_enabled:
    print_phantoms()
sys.exit(1)
PY
else
  echo "warning: python3 not found — falling back to line-only output;" >&2
  echo "         sub-line region misses will not be listed." >&2
  cargo +nightly llvm-cov nextest "${scope[@]}" \
    --all-features \
    --ignore-filename-regex "$IGNORE_REGEX" \
    --show-missing-lines \
    --fail-uncovered-lines 0 \
    --fail-uncovered-regions 0 \
    --fail-under-functions 0
fi
