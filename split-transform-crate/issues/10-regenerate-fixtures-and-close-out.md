# 10 — Regenerate fixtures, renumber the layer list, and close out

**What to build:** Settle everything that can only be settled once all the moves
have landed, and prove the work achieved what it set out to.

Two generated artifacts may be stale. Snapshot files are keyed by the path of
the test that produced them, so any test that moved may need its snapshots
regenerated — mechanically, with no source change alongside. Separately, a
generation chain crosses crates: Rust test sources feed a harvested parity
corpus in the compiler package, which generates a committed fixture in the value
parser, and that package's pre-test step checks it. Moving Rust test files
therefore invalidates a fixture in a crate this work never touched. Regenerate
it; never hand-edit it.

Then renumber the documented layer list end to end — the new crates shift the
numbering, and it can only be made consistent once they all exist — and record
the closing measurements against the baseline.

Two notes on the baseline. The `pre-split` criterion baseline no longer exists,
so bench comparisons are A/B runs against `develop` rather than diffs against a
saved baseline. And the incremental-check measurement in
[`../baseline.md`](../baseline.md) is *"add an item to `state_manager.rs`"* --
that file now lives in `crates/stylex-state/`, not in the transform. The
measurement stays valid and is the one the split most directly aims at, so keep
taking it; just do not go looking for the file where the baseline recorded it.

**Blocked by:** 09 — Re-home the evaluator's tests and benches.

**Status:** resolved

- [x] Any stale snapshots are regenerated, in a change containing no source edits.
- [x] The generated value-parser fixture is regenerated through its generator and the pre-test check passes.
- [x] The documented layer list is renumbered and every crate appears exactly
      once, including the two the spec did not plan: `stylex-state` and
      `stylex-declarations`.
- [x] The context map lists every new crate and no stale row remains.
- [~] No artifact anywhere asserts a porting or mirroring relationship with
      another implementation. Five doc comments calling this project "the port"
      are reworded. What remains is the `Unreachable port` domain term in
      `stylex-css-parser`, which is a rename of a defined term rather than a
      wording fix; [ticket 18](./18-rename-the-unreachable-port-term.md) closes
      it. Third-party attribution and test provenance stay, for the reasons in
      the close-out.
- [x] The full workspace suite is green in debug.
- [x] The compiler addon is rebuilt and the JavaScript suite passes against it.
- [x] The public entry points the compiler consumes are confirmed unchanged.
- [x] Coverage passes, and every crate still on an exclusion list has a reason
      recorded. This will not be the transform alone: `stylex-state` is excluded
      in all three lists and stays there until
      [ticket 11](./11-cover-the-state-crate.md) comes off the backlog. Say so
      in the close-out rather than leaving the list looking accidental.
- [x] A before/after table is recorded: largest crate size, excluded-from-coverage lines, cold build, incremental check after touching the state manager.

## What landed

### Generated artifacts

Both generation chains were already current, so neither needed a regeneration
commit:

- `parity:harvest:check` — "828 declarations — corpus is up to date."
- `generate:value-parser-cases:check` — no diff against the committed
  `postcss-value-parser/src/tests/cases.rs`.

The evaluator's move (tickets 08, 09, 13) carried no insta snapshot with it, so
no snapshot key moved. What was stale is older: every `source:` header still
named `crates/stylex-shared`, the crate name before the rename to
`stylex-transform`. Regenerated with `INSTA_FORCE_UPDATE=1`; the only lines that
changed are the `source:` headers and the `assertion_line` metadata insta no
longer writes. No expected value changed.

Two snapshots under `transform_override_vars_test/snapshots/` were keyed on a
test module named `stylex_transform_override_vars_test`, which does not exist —
no test read them. Deleted. The live pair beside them holds the same output.

### Layer list and context map

Both were already complete and correct, so this ticket changed neither. Checked
mechanically rather than by eye:

- The layer list in `guidelines/STRUCTURE.md` names all 25 crates, each exactly
  once, `stylex-state` at layer 9 and `stylex-declarations` at layer 10.
- Every layer claim holds against the real `[dependencies]` of each crate: no
  crate depends on a crate at its own layer or above.
- Every crate has a `CONTEXT.md`, every one has a row in `CONTEXT-MAP.md`, and
  no row names a directory that is gone.

### Porting and mirroring claims

Five doc comments called this project "the port" or "this port". Reworded to
name the compiler. Three kinds of statement were deliberately left alone,
because removing them would delete a fact the repo needs rather than a claim
about its identity:

- **Third-party attribution.** `stylex-styleq` and `postcss-value-parser` are
  ports of separate libraries, and `NOTICE.md` is the licensing record that says
  so. The map row and the crate docs must agree with it.
- **Test provenance.** A comment saying which upstream test file a case came
  from records where the expectation was obtained. That is evidence, not
  self-description.
- **The `Unreachable port` domain term** in `stylex-css-parser/CONTEXT.md`.
  Renaming a defined term is a domain-model change and needs its own ticket.

### Coverage and its exclusion list

Green: 100.00% of regions, functions and lines, 4709 tests, 0 uncovered.

Six crates are excluded, and the three lists that must agree — the
`test:coverage:workspace` script in the root `package.json`, `EXCLUDED_CRATES`
in `scripts/coverage-missing.sh`, and the `case` in
`scripts/packages/test/coverage.sh` — hold the same six.

Two of the six are this work's own doing and are a holding position, not a
judgement:

- `stylex_state` — excluded until [ticket 11](./11-cover-the-state-crate.md)
  comes off the backlog. Covered transitively through the transform today.
- `stylex_evaluator` — the same, until
  [ticket 15](./15-cover-the-evaluator-crate.md).

The other four (`stylex_logs`, `stylex_compiler_rs`, `stylex_test_parser`,
`stylex_transform`) predate this work and keep the reasons already recorded in
`guidelines/STRUCTURE.md`.

### The compiler's own entry points

`crates/stylex-rs-compiler/src/` is byte-identical to `develop`. Nine crate
extractions moved 37k lines out from under it and its source needed no edit, so
every path and signature it consumes is unchanged by construction. Its only
diff against `develop` is a comment in `Cargo.toml` and the regenerated parity
corpus.

The addon was rebuilt and the JavaScript suite run against it: 99 of 99 Turbo
tasks green.

### Before and after

Baseline is `e8887ab8f`, the commit [`../baseline.md`](../baseline.md)
describes. Same machine, same `dev` profile.

| Measurement | Baseline | Now | Change |
| --- | ---: | ---: | --- |
| Largest crate, `src/` lines | `stylex-transform` 60708 | `stylex-css-parser` 53624 | the transform is no longer the largest |
| `stylex-transform`, `src/` lines | 60708 | 23699 | -61% |
| Lines the coverage gate never measures | 34304, 4 crates | 31744, 6 crates | -7% |
| Cold build | 109.88 s | 117.96 s | +7%, inside the noise band |
| Incremental check, state-manager edit | 0.87 s, 2 crates re-checked | 1.50 s, 5 crates re-checked | slower — see below |
| Incremental check, transform edit | — | 0.68 s, 2 crates re-checked | new probe |
| Full suite | 8072 passed | 8188 passed | +116 |
| Coverage | 100.00% | 100.00% | held |

Line counts use the baseline's method: every `*.rs` file under `src/`, inline
`#[cfg(test)]` blocks included. The coverage row counts only what the gate can
see — `src/` less any path matching `(tests?|benches?|examples)/`.

**The state-manager probe got slower, and that is the honest reading.** At the
baseline the file sat inside `stylex-transform`, near the top of the graph, so
editing it re-checked two crates. It now sits at layer 9, with
`stylex-declarations`, `stylex-evaluator`, `stylex-transform` and
`stylex-rs-compiler` above it — five crates re-check, and the wall clock nearly
doubles. Moving a file down a layer moves everything above it into its rebuild
fan-out.

The measurement the split does improve is the one it was aiming at, taken at the
place the code now is: editing `stylex-transform` re-checks the same two crates
as before, but over 61% less source, and costs 0.68 s against the baseline's
0.87 s. The baseline picked the state manager because it was then the worst case
inside the transform; after the split those are two different questions, so both
are recorded.

The cold build's +7% sits inside the ~10% band the baseline declares as noise,
and the two runs did not start from the same disk state — `cargo clean` removed
2.2 GiB at the baseline and 72.2 GiB here, because this tree also holds coverage
and bench trees. Treat the row as "no measurable change".

### Benches

Not re-run. The `pre-split` criterion baseline no longer exists, the evaluator
benches changed crate and so lost their identity, and adding crates shifts every
LTO bench regardless of the code. Bench work is owned by
[ticket 16](./16-measure-the-crate-type-change.md) and
[ticket 17](./17-link-mimalloc-in-every-bench.md), both on the backlog.

### Gates

| Gate | Result |
| --- | --- |
| `cargo test --workspace --all-features` | green, 8188 passed, 0 failed, 32 binaries |
| `cargo clippy --workspace --all-features --all-targets` | green, 0 warnings |
| `cargo fmt --all --check` | clean |
| `cargo build --workspace --all-features` cold | green, 0 warnings |
| `scripts/coverage-missing.sh` | green, 100% on all three axes |
| `pnpm test` | green, 99 of 99 tasks |

### One thing the review turned up

A performance review of the whole split found `DiagnosticState` taken as
`&dyn` / `&mut dyn` at fourteen signatures in `stylex-diagnostics`. The trait
exists only to keep that crate from naming the state manager, and a generic
bound does that just as well, so the vtable bought nothing. It cost something:
the source-map annotation path asks the state three questions per style
namespace on its cache-hit route.

Changed to a generic bound in `1b422ddea`. Paired `transform_debug_bench` runs
on one machine: all twelve measurements moved the same way, 0.1% to 2.8%
faster, median about 0.7%. Criterion calls one an improvement and the rest
noise; the uniform sign across twelve is the signal. The trait doc had claimed
the dispatch was off the hot path, which was wrong, and now states the rule
instead.

The same review confirmed the three things most likely to have gone wrong in a
split this size, and none had: only `stylex-rs-compiler` declares a `cdylib`,
fat LTO still covers every new crate on the shipped profile, and no crate
reaches for a `std` hash map.

### Commits

| Commit | What |
| --- | --- |
| `dd4768a2c` | Snapshot header refresh, two dead snapshots deleted |
| `767a99dc1` | Doc rewording: the crates describe themselves |
| `1b422ddea` | `DiagnosticState` by generic bound |
