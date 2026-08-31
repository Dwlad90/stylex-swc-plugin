# 02 — Record the pre-split baseline

**What to build:** There is no defect to reproduce here, so the pre-flight for
this refactor is a measurement. Without it, "the split improved maintainability"
is unfalsifiable and no later ticket can prove it did not regress performance or
coverage.

Capture, at a named commit, the numbers every later ticket is measured against,
and store them beside this spec so any agent picking up a later ticket can diff
against them without re-deriving anything.

**Blocked by:** None — can start immediately.

**Status:** ready-for-human

- [x] Full suite recorded green, run directly — never piped into a pager or tail, or the exit code is the pager's.
- [x] All seven criterion benches recorded with machine and profile noted.
- [x] Coverage output saved, including the current exclusion list verbatim.
- [x] Cold build time recorded.
- [x] Incremental check time recorded after touching the state manager.
- [x] Source-line counts recorded per crate, so the end-state table has a starting point.
- [x] The commit the measurements describe is recorded explicitly.
- [x] Documentation only — no source changes in this ticket.

The result is [`../baseline.md`](../baseline.md), with raw logs in
[`../baseline/`](../baseline/). Everything describes commit `e8887ab8f`.

Nothing here is committed: `.scratch/` is a symlink outside the repository, so
the tracker and the baseline live beside the working copy rather than in it.

## Comments

### The coverage gate is red at the baseline, and ticket 01 made it red

This is the finding that matters, and it changes what the later tickets face.

`scripts/coverage-missing.sh` exits 1 with 23 uncovered regions in three files:
`stylex-ast/src/ast/imports.rs` and `.../source_file.rs`, and
`stylex-structures/src/pre_rule_value.rs`. All three were added by `5ba60950a`,
the second commit of ticket 01.

Nothing regressed in behaviour. What happened is that ticket 01 moved code down
out of `stylex_transform` — which the gate excludes — into two crates the gate
measures. The code was never covered; it was merely never looked at. The move is
right and the spec asks for it. The consequence is that **every extraction in
this sequence moves code into the gate**, so each one should be expected to need
tests for code whose behaviour did not change.

Ticket 01's own notes list a build, test, lint and format run, but no coverage
run, which is why this went unseen. Tickets 03 and later all carry "coverage
gate still passes" as a criterion, so none of them could start until these 23
regions were covered.

**Closed by `be48d03d1`**, with real tests rather than an exclusion. The gate
now reports 100% of regions, functions and lines across the workspace. The
baseline table keeps both numbers, so the red state at `e8887ab8f` stays on
record.

### The baseline is taken after ticket 01, not before it

The spec says "recorded before any code moves". Ticket 01 had already landed
when this ticket started. Re-measuring at the merge-base would describe a tree
no later ticket is ever compared against, so the baseline is taken at `HEAD`
and the commit is named everywhere. Ticket 01 is prefactoring — it creates no
crate — so the split proper still has not started at this point.

### Three measurement traps, all of which produced a wrong number first

Recorded in `baseline.md` beside the numbers they affect, because each one
silently yields a plausible-looking result:

- **The bench package id is `stylex_transform`, not `stylex-transform`.** The
  hyphenated directory name fails with "package ID specification did not match
  any packages".
- **Neither `cargo bench -p …` nor `--benches` can save a criterion baseline.**
  Cargo also runs the crate's lib test harness as a bench target, and that
  harness rejects `--save-baseline` with "Unrecognized option". Each of the
  seven targets has to be named with `--bench`.
- **A whitespace-only touch does not measure an incremental build.** rustc
  incremental compilation reuses everything when the code is unchanged, so the
  ticket's literal "touch the state manager" returns in no-op time. rustc also
  caches both sides of a repeated edit, so a probe has to be unique per run.

### The evaluator crate already exists

`stylex-evaluator` is present at 892 src and 401 test lines. Ticket 07 is
written as if it were seeding a new crate. Check what is already there before
starting it.
