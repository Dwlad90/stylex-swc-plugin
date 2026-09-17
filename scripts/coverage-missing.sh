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
# ROBUSTNESS FIXES over a naive `--show-missing-lines` gate:
#   1. Regions, not just lines — sub-line gaps are reported with line:col.
#   2. Monomorphization merge — regions are summed across generic instantiations
#      by source coordinate. A generic fn instantiated by a type that bails out
#      early (often a test mock) leaves per-instantiation gaps that llvm-cov
#      counts as uncovered even though the source line *is* exercised by another
#      instantiation. A region is reported only when no instantiation runs it;
#      the delta from llvm-cov's raw count is explained in a note.
#   3. Toolchain drift — the nightly compiler decides how regions are counted,
#      and continuous integration installs the newest nightly on every run. An
#      older local nightly measures fewer regions, so this report can read clean
#      while the same gate fails in continuous integration. The version in use is
#      printed, and an available update is a warning (`--skip-toolchain-check`
#      turns the network lookup off).
#   4. Gate parity — llvm-cov does not gate on the merge of fix 2. It scores a
#      function on its best-covered instantiation, so a region still counts
#      against `--fail-uncovered-regions 0` unless ONE instantiation runs it:
#      two instantiations that each run half a function leave a gap even though
#      every line was executed. That arithmetic is reproduced here and the
#      regions behind it are named, so a failing gate always has a location to
#      act on rather than a bare count.
#   5. Scope filtering — cargo-llvm-cov's target dir is stateful: a `-p <crate>`
#      run can fold in leftover instrumented object files from an earlier
#      full-workspace run (e.g. dependency crates), producing a noisy,
#      non-deterministic file list. The report is filtered to the requested
#      scope so the output is stable no matter what the target dir holds.
#   6. Stale mappings — the filter above keeps another crate's *files* out of
#      the report, and cannot keep another crate's *objects* from answering for
#      this one. A region's position is recorded in the object it was compiled
#      into, and a run rebuilds only the objects of the crates in its scope, so
#      a generic of this crate that was compiled into a crate above carries the
#      positions it had when that crate was last built. The report reads
#      plausible and is wrong -- a file at 100% named as 50%, with its misses on
#      lines that hold doc comments. So every position is checked against the
#      source it names, and a mapping that disagrees is refused before a table,
#      a miss list or an HTML report is produced.
#
#      Refused rather than cleaned automatically. A clean is a full instrumented
#      rebuild, and the scope of the run is not enough to know one is needed:
#      `pnpm test:coverage:workspace` writes the same object directory, so a
#      repeated `-p <crate>` can read foreign objects without the scope having
#      changed at all. Reading the source is the whole answer, and it costs one
#      pass over the files the report covers.
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
#   scripts/coverage-missing.sh --strict-phantoms # deprecated alias for --show-phantoms
#   scripts/coverage-missing.sh --skip-toolchain-check # do not look for a newer nightly
#   scripts/coverage-missing.sh --html          # also write an HTML report (second run)
#   scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
#   scripts/coverage-missing.sh -h | --help
#
# EXIT STATUS
#   0  every measured source region is exercised by at least one test
#   1  one or more source regions are unexercised, or a region counts against
#      the coverage gate because no single instantiation runs it. Both are
#      printed above as a `file:line:col` list.
#   2  the arguments could not be read (unknown option, missing crate name)
#   3  the coverage mapping does not match the source it names, so nothing is
#      reported. Run `cargo +nightly llvm-cov clean --workspace` and measure
#      again.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Crates excluded from workspace coverage. Five lists must agree: this one, the
# `test:coverage:workspace` script in the root package.json, the `case` in
# scripts/packages/test/coverage.sh, `EXCLUDED` in
# scripts/git/crate-coverage-runner.test.mjs, which asserts that `case`, and the
# rows under "Excluded from Coverage" in guidelines/STRUCTURE.md.
# scripts/git/coverage-exclusions.test.mjs compares all five and names the one
# that disagrees. A row is either permanent or names the ticket that removes it
# -- see "Excluded from Coverage" in guidelines/STRUCTURE.md.
EXCLUDED_CRATES=(
  stylex_logs        # permanent
  stylex_compiler_rs # permanent
  stylex_test_parser # permanent
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
  scripts/coverage-missing.sh --strict-phantoms # deprecated alias for --show-phantoms
  scripts/coverage-missing.sh --skip-toolchain-check # do not look for a newer nightly
  scripts/coverage-missing.sh --html          # also write an HTML report (second run)
  scripts/coverage-missing.sh --open          # write + open the HTML report in a browser
  scripts/coverage-missing.sh -h | --help

EXIT STATUS
  0  every measured source region is exercised, and each by a single instantiation
  1  a source region is unexercised, or no single instantiation runs it (the
     file:line:col list is printed above)
  2  the arguments could not be read
  3  the coverage mapping is stale; clean and measure again
EOF
  exit "${1:-0}"
}

package=""
html=0
open=0
show_phantoms=0
toolchain_check=1

while [ $# -gt 0 ]; do
  case "$1" in
    -h | --help) usage 0 ;;
    --show-phantoms) show_phantoms=1 ;;
    --strict-phantoms) show_phantoms=1 ;;
    --skip-toolchain-check) toolchain_check=0 ;;
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

# Name the compiler that measures this run, and say when a newer one exists.
#
# Region counting is a property of the nightly compiler, and continuous
# integration installs the newest nightly every time. An older local nightly can
# merge regions that the newer one counts apart, which reads here as full
# coverage while the same gate fails in continuous integration. The version is
# always printed, so a report can be compared with the one in the job log.
report_toolchain() {
  local version
  version="$(rustc +nightly --version 2>/dev/null || true)"

  if [ -z "$version" ]; then
    echo "warning: no nightly toolchain found -- run 'rustup toolchain install nightly'" >&2
    return 0
  fi

  echo "==> Toolchain: $version"

  # The lookup needs the network, so it is skippable and never fatal.
  [ "$toolchain_check" -eq 1 ] || return 0

  local status
  status="$(rustup check 2>/dev/null | grep '^nightly' || true)"

  case "$status" in
    *"update available"*)
      echo "warning: a newer nightly is available -- $status" >&2
      echo "         Continuous integration builds with the newest nightly, and the" >&2
      echo "         region count depends on the compiler, so this report can disagree" >&2
      echo "         with it. Run 'rustup update nightly' to compare like with like." >&2
      ;;
  esac
}

report_toolchain

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
  # `${a[@]}` on an empty array is an unbound variable under bash 3.2 with
  # `set -u`, and a temporary row leaves one day, so the list can be empty.
  scope=(--workspace ${WORKSPACE_EXCLUDES[@]+"${WORKSPACE_EXCLUDES[@]}"})
  scope_mode="workspace"
  scope_value=""
  for crate in "${EXCLUDED_CRATES[@]}"; do
    scope_value+="${crate//_/-},"
  done
  echo "==> Coverage for workspace (excluding non-instrumented crates)"
fi

tmp_json="$(mktemp "${TMPDIR:-/tmp}/coverage-missing.XXXXXX")"
tmp_log="$(mktemp "${TMPDIR:-/tmp}/coverage-missing.XXXXXX")"
# Inline rather than a named function: the script now ends on an explicit
# `exit`, and past one of those shellcheck stops reading a `trap` as a call and
# reports the function as never invoked.
trap 'rm -f "$tmp_json" "$tmp_log"' EXIT

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
  cargo +nightly llvm-cov clean --workspace
  set +e
  run_coverage
  status=$?
  set -e
fi

if [ "$status" -ne 0 ]; then
  echo "error: coverage run failed (see output above)" >&2
  exit "$status"
fi

# Render the summary table and the exact uncovered locations, and set the exit
# status, all from the JSON export.
report_status=0

if command -v python3 >/dev/null 2>&1; then
  set +e
  python3 - "$tmp_json" "$REPO_ROOT" "$IGNORE_REGEX" "$scope_mode" "$scope_value" "$show_phantoms" <<'PY'
import json
import os
import pathlib
import re
import sys
from collections import defaultdict

json_path, repo_root, ignore_regex, scope_mode, scope_value, show_phantoms_arg = sys.argv[1:7]
ignore = re.compile(ignore_regex)
show_phantoms_enabled = show_phantoms_arg == "1"

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


# llvm-cov region kinds: 0 = Code (the kind region coverage is based on),
# 1 = Expansion, 2 = Skipped, 3 = Gap, 4 = Branch, 5/6 = MC/DC. Only "Code"
# regions are counted, matching `--fail-uncovered-regions`.
CODE_KIND = 0


def measured_region_starts():
    """Where each measured region begins, as the export records it."""
    for function in export.get("functions", []):
        filenames = function.get("filenames", [])

        for region in function.get("regions", []):
            if len(region) < 8 or region[7] != CODE_KIND:
                continue

            file_id = region[5]
            filename = filenames[file_id] if file_id < len(filenames) else ""

            if filename in measured:
                yield filename, region[0], region[1]


def stale_coordinates():
    """Reported positions that the source they name cannot hold.

    The coverage mapping lives in the object files, not in the profile data, so
    a run that did not rebuild every object can report a position from an older
    revision of a file. A `-p <crate>` run is how that happens: the objects of
    the crates above it are not rebuilt, and a generic from this crate compiled
    into one of them carries this crate's coordinates with it.

    Nothing in the export says which revision a position came from, so the
    source is asked instead. A `Code` region starts at a token, so it cannot
    start on a blank line and it cannot start inside a line comment -- and a
    string literal's region starts at its opening quote, so a raw string
    holding `//` is not a reading of this either. One that does start there
    came from a file that has since been edited.

    Only those two readings, because they are the two that cannot be anything
    else. A leading `*` is not one of them -- `*count -= 1` is a line of code
    that begins the way a wrapped doc comment does.

    Every measured region is asked, not only the unexercised ones. A stale
    mapping reports an unexercised region as covered just as readily, and that
    is the answer a reader would act on without noticing.
    """
    stale = []
    source = {}

    for filename, line_start, col_start in measured_region_starts():
        if filename not in source:
            try:
                source[filename] = (
                    pathlib.Path(filename).read_text(errors="replace").splitlines()
                )
            except OSError:
                source[filename] = None

        lines = source[filename]

        if lines is None:
            continue

        if line_start > len(lines):
            stale.append((filename, line_start, col_start, "past the end of the file"))
            continue

        text = lines[line_start - 1]

        # Columns are byte offsets, so the line is measured in bytes too. A line
        # carrying a multi-byte character is longer in bytes than in characters,
        # and the character count would call a position near its end stale.
        if col_start > len(text.encode("utf-8")) + 1:
            stale.append((filename, line_start, col_start, "past the end of the line"))
        elif not text.strip() or text.lstrip().startswith("//"):
            stale.append((filename, line_start, col_start, "a line that carries no code"))

    return stale


def refuse_if_stale():
    """Refuses to answer at all from a mapping the source disagrees with.

    Asked before anything is printed, because a stale mapping still renders a
    summary table that reads perfectly plausible -- the one that sent a reader
    chasing a file at 50% that was really at 100%.
    """
    stale = stale_coordinates()

    if not stale:
        return

    print(
        "\nerror: the coverage mapping is stale -- it names positions this source\n"
        "       cannot hold, so every count in it is from an older revision of\n"
        "       these files. Nothing is reported, because a stale mapping still\n"
        "       renders a table that reads perfectly plausible. Run:\n\n"
        "         cargo +nightly llvm-cov clean --workspace\n\n"
        "       and measure again. This happens after a change that moves lines,\n"
        "       because a `-p <crate>` run rebuilds that crate and not the\n"
        "       objects of the crates above it.\n",
        file=sys.stderr,
    )

    for filename, line, col, why in stale[:5]:
        print(f"       {rel(filename)}:{line}:{col} is {why}", file=sys.stderr)

    if len(stale) > 5:
        print(f"       ... and {len(stale) - 5} more", file=sys.stderr)

    sys.exit(3)


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

refuse_if_stale()


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


# ── What the coverage gate counts ────────────────────────────────────────────
# `--fail-uncovered-regions 0` reads llvm-cov's per-file summary, and that
# summary is NOT the merge above. llvm-cov collects the records of one source
# function -- the generic shell plus every monomorphization -- into an
# instantiation group, and scores the group on its best-covered record. So a
# region counts against the gate unless a SINGLE instantiation runs it: two
# instantiations that each run half of a function leave a gap even though every
# line of the source was executed by one of them.
#
# The arithmetic below reproduces that summary exactly, which is what lets this
# script name the regions behind a failing gate instead of only counting them.
# A group is keyed by its file and the start of its first region, which is the
# function's own position in the source.
group_records = defaultdict(list)
for function in export.get("functions", []):
    filenames = function.get("filenames", [])
    per_file = defaultdict(list)
    for region in function.get("regions", []):
        if len(region) < 8 or region[7] != CODE_KIND:
            continue
        file_id = region[5]
        filename = filenames[file_id] if file_id < len(filenames) else ""
        if filename not in measured:
            continue
        per_file[filename].append(region)
    for filename, regions in per_file.items():
        start = min((region[0], region[1]) for region in regions)
        group_records[(filename, start)].append((function.get("name", ""), regions))

# Regions no instantiation runs are already reported on their own, so the gate
# section names only what the merge cannot explain.
uncovered_set = set(uncovered)


def gate_gaps():
    """Every function the gate counts a region against, and who comes closest.

    llvm-cov takes the covered count and the region count of a group as the
    maximum each reaches over the group's records, so the records that run the
    most regions are the ones the gate is scored on. Each of those is reported
    with what it still misses: a reader can then see whether one more case
    closes the whole group, or whether the instantiations have to be collapsed
    into one."""
    gaps = []

    for (filename, start), records in sorted(group_records.items()):
        scored = [
            (sum(1 for region in regions if region[4] > 0), len(regions), name, regions)
            for name, regions in records
        ]
        best = max(row[0] for row in scored)
        total = max(row[1] for row in scored)

        if best >= total:
            continue

        leaders = []
        for covered, _, name, regions in sorted(scored, key=lambda row: row[2]):
            if covered != best:
                continue
            missed = [
                region
                for region in sorted(regions)
                if region[4] == 0
                and (filename, region[0], region[1], region[2], region[3]) not in uncovered_set
            ]
            if missed:
                leaders.append((name, missed))

        if leaders:
            gaps.append((filename, start, best, total, leaders))

    return gaps


_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_SNAKE = re.compile(r"[a-z][a-z0-9_]*")
_CAMEL = re.compile(r"[A-Z][a-z][A-Za-z0-9]*")
# Std/crate-root path noise we drop to keep the hint focused on our own code.
_DROP = {"core", "alloc", "std", "option", "result", "string"}


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

gate = gate_gaps()
gate_count = sum(total - best for _, _, best, total, _ in gate)


def print_gate_gaps():
    """Name the regions the gate counts, function by function."""
    print(
        "\nRegions the coverage gate counts (`--fail-uncovered-regions 0`):\n"
        "\n"
        "llvm-cov scores a function on its best-covered instantiation, so a region\n"
        "counts here unless one instantiation runs it. The lines below were each run\n"
        "by some instantiation, just never all by the same one. Close a group by\n"
        "driving a single instantiation through every line of the function.\n"
    )

    for filename, (line, _col), best, total, leaders in gate:
        print(f"  {rel(filename)}")
        print(
            f"    fn at line {line}: the best instantiation runs {best} of {total} region(s)"
        )
        for name, missed in leaders:
            print(f"      {readable_symbol(name)} misses")
            for line_start, col_start, line_end, col_end, *_ in missed:
                if line_start == line_end:
                    print(f"        line {line_start:<6} cols {col_start}-{col_end}")
                else:
                    print(f"        lines {line_start}-{line_end:<6} cols {col_start}-{col_end}")
        print()

    print(
        f"{gate_count} region(s) counted against the gate across "
        f"{len({gap[0] for gap in gate})} file(s)."
    )


# llvm-cov's own (per-instantiation) uncovered-region tally, for the note below.
raw_notcovered = sum(
    f.get("summary", {}).get("regions", {}).get("notcovered", 0) for f in file_entries
)

if raw_notcovered > len(uncovered):
    suffix = ""
    if not show_phantoms_enabled:
        suffix = "\n      Re-run with --show-phantoms to print every per-instantiation gap."
    print(
        f"\nnote: llvm-cov counts {raw_notcovered} uncovered region(s), of which "
        f"{len(uncovered)} are unexercised by every test.\n"
        "      The rest are generic monomorphization: the gate scores a function on its\n"
        "      best-covered instantiation, so a region it counts can still have been run\n"
        f"      by another instantiation. Each one is named below.{suffix}"
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

if uncovered:
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

    # Compact, grep-friendly `file: line, line, ...` list of the region starts.
    print("Uncovered lines (compact):")
    for filename in sorted(by_file):
        start_lines = sorted({line_start for line_start, *_ in by_file[filename]})
        print(f"  {rel(filename)}: " + ", ".join(str(line) for line in start_lines))

    region_count = sum(len(v) for v in by_file.values())
    print(f"\n{region_count} uncovered region(s) across {len(by_file)} file(s).")

if gate:
    print_gate_gaps()

if show_phantoms_enabled:
    print_phantoms()

sys.exit(1)
PY
  report_status=$?
  set -e
else
  echo "warning: python3 not found — falling back to line-only output;" >&2
  echo "         sub-line region misses will not be listed, and the mapping is" >&2
  echo "         not checked against the source it names." >&2
  # Narrowed to the two answers this branch can give. Reporting cargo's own
  # status would let one of its other codes read as a status of ours -- 3 says
  # the mapping is stale, and this branch never looked.
  if cargo +nightly llvm-cov nextest "${scope[@]}" \
    --all-features \
    --ignore-filename-regex "$IGNORE_REGEX" \
    --show-missing-lines \
    --fail-uncovered-lines 0 \
    --fail-uncovered-regions 0 \
    --fail-under-functions 0; then
    report_status=0
  else
    report_status=1
  fi
fi

# A stale mapping is reported and nothing else: an HTML report would read as
# plausible for the same reason the table does, and `--open` hands it to a
# browser.
if [ "$report_status" -eq 3 ]; then
  exit 3
fi

# Optional HTML report. `--html` cannot share an invocation with `--json`, so
# it costs a second instrumented run; it is opt-in and rare. Written after the
# report above, so a mapping the source disagrees with never reaches it.
if [ "$html" -eq 1 ]; then
  html_flags=(--html)
  [ "$open" -eq 1 ] && html_flags+=(--open)
  cargo +nightly llvm-cov nextest "${scope[@]}" \
    --all-features \
    --ignore-filename-regex "$IGNORE_REGEX" \
    "${html_flags[@]}"
fi

exit "$report_status"
