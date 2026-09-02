# Pre-split baseline

Every number below describes commit **`e8887ab8f`** on branch
`feat_split-transform-crate` (`docs(guidelines): state what a re-export may and
may not do`). Ticket 01 has already landed at that commit; the crate split
itself has not started, so this is the point every later ticket is measured
against.

Raw logs are in [`baseline/`](./baseline/). Re-run any step and diff its log.

## Machine and profile

| Item | Value |
| --- | --- |
| Machine | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1 |
| Toolchain | rustc 1.97.0 / cargo 1.97.0 (nightly for coverage) |
| Profile | `dev` (debug) everywhere except the benches, which cargo builds in `bench` |
| Target dir | per-worktree, no `CARGO_TARGET_DIR` override |

Benches and build times are wall-clock on a shared laptop. Treat a change under
about 10% as noise, and re-run before you believe a single outlier.

## Full suite

`cargo test --workspace --all-features`, run directly — not piped, or the exit
status would be the pager's.

| Item | Value |
| --- | --- |
| Result | **green**, exit 0 |
| Tests | 8072 passed, 0 failed, across 27 test binaries |
| Wall clock | 81.94 s real / 549.77 s user, including compilation |

Log: [`baseline/test-suite.log`](./baseline/test-suite.log).

## Build times

| Measurement | Wall clock |
| --- | --- |
| Cold build — `cargo clean`, then `cargo build --workspace --all-features` | **109.88 s** (user 849.65 s, sys 38.82 s) |
| Incremental check — add an item to `state_manager.rs`, then `cargo check --workspace --all-features` | **0.87 s** (0.82 s reported by cargo) |
| Incremental check — comment-only edit to the same file | 0.69 s |

The cold build emits **zero warnings**. `cargo clean` removed 7061 files /
2.2 GiB, so the on-disk target directory this measurement starts from is the
build artefacts only, not the coverage and release trees that also live there.

### Against the branch, at ticket 16

Re-measured on the same machine, one target directory per configuration, source
identical within each pair. Full numbers and logs are in
[`bench/ticket-16.md`](./bench/ticket-16.md).

| Measurement | Pre-split (`e8887ab8f`) | Branch, at ticket 16 |
| --- | --- | --- |
| Cold build, `dev` | 109.88 s | 110.79 s to 113.11 s |
| Workspace-only rebuild, `dev` | not measured | 8.0 s |
| Workspace-only rebuild, `release` | not measured | 47.5 s |
| Dynamic libraries the workspace emits | 20 | **1** |
| Published `.node` | not measured | **19,538,192 B** |

The cold build is unchanged, because 400-odd third-party crates dominate it. The
**workspace-only rebuild** is the number the split moves, and it is the one to
quote from here on: it cleans this workspace's crates and leaves every
dependency built, so a crate-type or layer change is the whole of the
difference. Dropping the nineteen unlinked `cdylib`s took about a quarter off
it, in both profiles.

The `.node` grew from 18,724,368 B when the addon still carried an unused
`rlib`. That `rlib` switched fat LTO off; removing it made 64 of 64 benchmark
fixtures faster by a median of 16.5%, and the extra 813 KB is inlined code.

The state manager is the file the split moves code out of, which is why the
incremental measurement touches it: it is the worst realistic case for rebuild
fan-out inside `stylex-transform`. Touching it re-checks two crates —
`stylex_transform` and `stylex_compiler_rs` downstream of it.

Two traps make this number easy to mis-measure, and both cost real time to find:

- **A whitespace-only touch measures nothing.** rustc incremental compilation
  reuses everything when the code is unchanged, so appending a blank line
  returns in the same time as a no-op check. Use an edit that adds an item.
- **rustc caches both sides of an edit.** Repeating the same probe text hits the
  incremental cache from the previous run and returns near-instantly. Make each
  probe unique, for example by appending a timestamp to the name.

A first `cargo check` after other build activity — a `cargo build` or a bench
compile — costs about **5.7 s** for `stylex_transform` alone, because it builds
check artefacts the other profile did not leave behind. That is a different
measurement; do not compare it against the 0.87 s above.

Logs: [`baseline/cold-build.log`](./baseline/cold-build.log),
[`baseline/incremental-check.log`](./baseline/incremental-check.log).

## Coverage

Command: [`scripts/coverage-missing.sh`](../../scripts/coverage-missing.sh),
which mirrors the CI gate `test:coverage:workspace`.

Exclusion list, verbatim — four crates, plus every path matching
`(tests?|benches?|examples)/`:

```
--exclude stylex_logs
--exclude stylex_compiler_rs
--exclude stylex_test_parser
--exclude stylex_transform
--ignore-filename-regex '(tests?|benches?|examples)/'
```

| Item | Value at `e8887ab8f` | Value now |
| --- | --- | --- |
| Result | **red**, exit 1 | **green**, exit 0 |
| Totals | 99.89% regions / 99.85% functions / 99.89% lines | 100.00% of all three |
| Uncovered | 23 regions across 3 files | none |
| Tests run under coverage | 4367 passed, 0 skipped | 4407 passed, 0 skipped |

The three files that were red, with their coverage at the baseline commit:

| File | Regions | Functions | Lines |
| --- | --- | --- | --- |
| `crates/stylex-ast/src/ast/imports.rs` | 0.00% | 0.00% | 0.00% |
| `crates/stylex-ast/src/ast/source_file.rs` | 80.49% | 100.00% | 82.76% |
| `crates/stylex-structures/src/pre_rule_value.rs` | 0.00% | 0.00% | 0.00% |

All three are at 100% since `be48d03d1`, which covers them with tests rather
than with an exclusion.

**The gate is red at the baseline, and ticket 01 made it red.** All three files
were added by `5ba60950a`, the second commit of ticket 01, which moved code down
out of the ungated `stylex_transform` into two gated crates. The move was
correct; what came with it is that code the gate never measured before is now
measured, and it arrives untested. Ticket 01's notes record a build, test, lint
and format run, but no coverage run, so this went unseen.

Every ticket from 03 onward carries "coverage gate still passes" as a criterion,
so this had to be cleared before any of them could start. The same trap waits at
each later extraction: **moving a file down a layer moves it into the gate.**
Expect to write tests for moved code even though the refactor changes no
behaviour.

Two things the closing commit found, both worth repeating at the next
extraction:

- **A bare panic assertion is not enough.** `catch_unwind(...).is_err()` also
  passes when an unrelated panic comes first, which hides the branch the test
  is there to reach. Assert on the panic message.
- **A platform-gated test module is an exclusion by another name.** Code behind
  `#[cfg(unix)]` tests goes uncovered on any other platform. Gate the helper
  that builds the platform-specific input, not the test.

Log: [`baseline/coverage.log`](./baseline/coverage.log).

## Criterion benches

Seven benches, all in `stylex-transform`, saved under the criterion baseline
name **`pre-split`**:

| Bench target | Groups | Moves with the evaluator |
| --- | --- | --- |
| `concatenation_chain_bench` | `ConcatenationChain` | no |
| `engine_fold_bench` | `EngineFoldColdStart`, `EngineFoldRoundTrip` | no |
| `evaluate_bench` | `EvaluatePerfFixtures` | **yes** |
| `evaluate_depth_bench` | `EvaluateDepth`, `StructuralKeyDepth`, `StructuralKeyFallback` | **yes** |
| `module_path_bench` | `ModuleWalk`, `SeenModuleSource`, `StructuralKey`, `StateManager`, `FullPipeline` | no |
| `transform_consumers_bench` | `TransformConsumers` | no |
| `transform_debug_bench` | `TransformDebugPath`, `TransformDebugNamespacesPerCall` | **yes** |

All seven ran green and saved 70 measurements. The medians are in
[`baseline/bench-summary.txt`](./baseline/bench-summary.txt), one line per
benchmark id; the full criterion output is in
[`baseline/benches.log`](./baseline/benches.log).

Re-run and diff with:

```bash
for b in concatenation_chain_bench engine_fold_bench evaluate_bench \
         evaluate_depth_bench module_path_bench transform_consumers_bench \
         transform_debug_bench; do
  cargo bench -p stylex_transform --bench "$b" -- --baseline pre-split
done
```

Two things about that command are not obvious, and both cost a failed run to
find:

- **The package id is `stylex_transform`, with an underscore.** The directory is
  `crates/stylex-transform`, with a hyphen. `-p stylex-transform` fails with
  "package ID specification did not match any packages".
- **Name each bench target.** Neither a bare `cargo bench -p …` nor `--benches`
  works: cargo also runs the crate's lib test harness as a bench target, and
  that harness rejects criterion's `--save-baseline` with "Unrecognized option".

Criterion baseline identities are per crate. The three evaluator benches move to
`stylex-evaluator` in the final commit, which resets their identity, so those
three need a manual same-machine before/after comparison rather than an
automatic diff. The other four diff normally.

Benches are not CI-gated, so this is a local step. One bench and the performance
fixture test are wall-clock flaky; re-run a lone failure before believing it.


## Source lines

Per crate, counted over `src/`, `tests/` and `benches/`. `src` includes inline
`#[cfg(test)] mod tests` blocks, so it is larger than the shipped library.

See [`baseline/source-lines.txt`](./baseline/source-lines.txt) for every crate.
The crate this work splits:

| Crate | src | tests | benches |
| --- | ---: | ---: | ---: |
| `stylex-transform` | 60708 | 48807 | 2977 |

Its `src/` breaks down as ([`baseline/transform-modules.txt`](./baseline/transform-modules.txt)):

| Module | Lines |
| --- | ---: |
| `shared/utils` | 37179 |
| `shared/structures` | 8453 |
| `shared/transformers` | 7282 |
| `transform` | 7151 |
| `shared/enums` | 517 |
| everything else | 122 |

The spec's end-state target is the transform dropping from about 32k to about
20k lines of *shipped* source, with roughly 13.8k moving into three new gated
crates. The 60708 figure above counts inline tests too, so compare the end state
against this same command, not against the spec's number.

One crate on the target list already exists: `stylex-evaluator`, at 892 src and
401 test lines. Ticket 07 seeds it, so check what is already there before
starting that ticket.
