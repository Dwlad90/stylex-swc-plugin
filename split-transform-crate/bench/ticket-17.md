# Ticket 17 — the re-baseline

**This is the reference point, and part of it is now closed.** Tickets 57, 58
and 59 changed what 195 of these measurements do, so read
[`ticket-57-58-59.md`](./ticket-57-58-59.md) first: it names every row whose
work changed and every id that changed with it. The rest of this record still
stands.

Every criterion number taken from here on compares against this record.
Numbers taken before it were measured against the system allocator and do not
compare with it; see
[ticket 17](../issues/17-link-mimalloc-in-every-bench.md) for why, and "The
allocator a bench measures" in `guidelines/PERFORMANCE.md` for the rule.

## What was measured

| Item | Value |
| --- | --- |
| Commit | `a4b520b45` on `develop` (`Bump version to 0.19.0-rc.3`) |
| Criterion baseline name | `mimalloc` |
| Bench targets | 14, in 6 crates |
| Measurements | 268, all green, no panic |
| Machine | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1 |
| Toolchain | rustc 1.97.0 (2d8144b78 2026-07-07) |
| Profile | `bench` |
| Target dir | per-worktree, no `CARGO_TARGET_DIR` override |

One script, not fourteen runs by hand. Measure, then summarise:

```sh
./.scratch/split-transform-crate/tools/rebaseline.sh mimalloc \
  .scratch/split-transform-crate/baseline/benches-mimalloc.log
python3 .scratch/split-transform-crate/tools/bench_medians.py \
  .scratch/split-transform-crate/baseline/benches-mimalloc.log \
  > .scratch/split-transform-crate/baseline/bench-summary-mimalloc.txt
```

Raw log: [`../baseline/benches-mimalloc.log`](../baseline/benches-mimalloc.log).
Medians, one line per benchmark id:
[`../baseline/bench-summary-mimalloc.txt`](../baseline/bench-summary-mimalloc.txt).

## The tree was green first

A baseline of a red tree measures nothing worth keeping, so both gates ran at
the same commit before any timing started.

| Gate | Result | Log |
| --- | --- | --- |
| `pnpm lint:crates:workspace` (clippy, `-D warnings`) | exit 0, **zero warnings** | [`clippy-mimalloc.log`](../baseline/clippy-mimalloc.log) |
| `pnpm test:crates:workspace` (nextest + doc tests) | exit 0, **2 of 2 tasks**, both a cache miss | [`test-suite-mimalloc.log`](../baseline/test-suite-mimalloc.log) |
| `cargo test --workspace --all-features` | exit 0, **9099 passed, 0 failed**, 31 test binaries | [`test-cargo-mimalloc.log`](../baseline/test-cargo-mimalloc.log) |

The turbo gate logs only errors, so its log carries the task result and not a
test count. The third row is the same suite run directly, for the count. Both
turbo tasks report a cache miss, so neither was served from cache.

## What the machine was doing

**It was not quiet when the session started, and the reason is worth keeping.**
Twenty orphaned `zsh` processes held the machine at 0% idle for 19 hours 16
minutes. Each ran `(while :; do :; done) &`, a CPU-load generator an earlier
session used to reproduce the flaky `stylex_transform_performance_test` under
load. Their parent died before `kill $loadpids` ran, so they were re-parented to
PID 1 and never stopped. Together they held 791% CPU on a 10-core machine, and
the load average read 821.

They were killed before the run. The run then started at **62% idle**, which
on ten cores is **3.8 cores busy**: a browser held about one, and the rest was
the system settling after 19 hours of saturation. That is not a silent machine,
and it is why `rebaseline.sh` now refuses to measure below 60% idle: the bar
catches saturation, which is the failure that actually happened, without
demanding a condition a desktop never meets. The gate reads one sample, before
the run, and the run then takes about 37 minutes unwatched.

Treat a change under about 10% as noise, as the pre-split baseline does, and
re-run a lone outlier before believing it. **That 10% is a convention, not a
measurement.** Criterion's own interval inside this run is tight -- 0.24% on
`Parse/corpus`, for one -- but nothing here was run twice, so cross-run noise
on this machine is unmeasured. The 34% figure in `guidelines/PERFORMANCE.md`
describes the JavaScript harness on hosted runners and does not transfer.

## Measurements per target

| Bench target | Crate | Measurements |
| --- | --- | ---: |
| `value_parser_bench` | `postcss-value-parser` | 7 |
| `css_types_bench` | `stylex-css-parser` | 72 |
| `properties_bench` | `stylex-css-parser` | 63 |
| `token_parser_bench` | `stylex-css-parser` | 28 |
| `css_generation_bench` | `stylex-css` | 6 |
| `normalize_value_bench` | `stylex-css` | 3 |
| `engine_fold_bench` | `stylex-evaluator` | 10 |
| `evaluate_bench` | `stylex-evaluator` | 8 |
| `evaluate_depth_bench` | `stylex-evaluator` | 10 |
| `performance_bench` | `stylex-styleq` | 16 |
| `concatenation_chain_bench` | `stylex-transform` | 4 |
| `module_path_bench` | `stylex-transform` | 20 |
| `transform_consumers_bench` | `stylex-transform` | 6 |
| `transform_debug_bench` | `stylex-transform` | 15 |
| **Total** | | **268** |

The pre-split baseline covered 70 measurements in one crate. This one covers
268 in six, because ticket 17 brought into the same run the four crates that
criterion could already measure but nobody had baselined. The crate the effort
split had also become two by then, which is the sixth.

## Against the pre-split baseline, and what the difference is not

All 70 pre-split benchmark ids still exist and were measured again. Sixty-four
of the seventy are faster.

| | Value |
| --- | --- |
| Shared ids | 70 of 70 |
| Faster | 64 |
| Median ratio, new / old | **0.861x** |
| Fastest | 0.391x (`TransformDebugNamespacesPerCall/prod/128`) |
| Slowest | 1.050x (`StateManager/new`) |

Per group, sorted by how far each moved:

| Group | n | Median new / old |
| --- | ---: | ---: |
| `EvaluatePerfFixtures` | 8 | 0.562x |
| `TransformDebugNamespacesPerCall` | 6 | 0.569x |
| `TransformDebugPath` | 6 | 0.668x |
| `EngineFoldColdStart` | 1 | 0.786x |
| `EngineFoldRoundTrip` | 9 | 0.809x |
| `SeenModuleSource` | 4 | 0.824x |
| `StructuralKeyFallback` | 2 | 0.854x |
| `FullPipeline` | 6 | 0.891x |
| `ModuleWalk` | 6 | 0.929x |
| `EvaluateDepth` | 4 | 0.953x |
| `TransformConsumers` | 6 | 0.962x |
| `StructuralKey` | 3 | 0.962x |
| `StructuralKeyDepth` | 4 | 0.999x |
| `ConcatenationChain` | 4 | 1.001x |
| `StateManager` | 1 | 1.050x |

**Do not read this table as the allocator's effect.** Three changes sit between
the two commits and each one moves these numbers:

1. the crate split itself, tickets 03 to 14;
2. dropping nineteen unlinked `cdylib`s, which switched fat LTO back on and
   made 64 of 64 benchmark fixtures faster by a median of 16.5%
   ([ticket 16](./ticket-16.md));
3. the allocator, ticket 17.

The table says how much of the recorded history the re-baseline invalidates,
which is the question this record exists to answer. Attributing any part of it
to one of the three needs an A/B across that change alone, on one machine, and
none of the three can be measured that way any more, because all three commits
are landed.

The one row that reads as a regression, `StateManager/new` at 1.050x, is a
single measurement 5% apart on a machine with a browser on it. That is inside
the noise this record declares.

**Each ratio is two point estimates divided, which is not a verdict.**
`guidelines/PERFORMANCE.md` ends its Verdict section with "Do not 'fix' it by
comparing point estimates", and that is what this table does: one run of each
side, no rounds, no bootstrap, no interval. It cannot be: the old side is a
saved summary of a commit whose code is gone. Read the table for scale only.

**The shape is consistent with the allocator, and that is all it is.** The three
groups that moved furthest — `EvaluatePerfFixtures`, and the two debug-transform
groups — are the allocation-heavy ones ticket 17 named in advance. They are also
the inlining-heavy ones the LTO change would move, and the ones the split moved
between crates. Three candidate causes point the same way, so the agreement
confirms none of them.

## Four things this run found

**Eleven benchmarks cannot take 100 samples in 5 seconds.** Criterion says so,
extends the target time itself and still collects 100 samples, so those numbers
stand like every other; the warnings are in the log. They are the slow fixtures
— `TransformDebugPath`, `TransformConsumers`, `FullPipeline`,
`ConcatenationChain/links/5` and `/10`, and
`EvaluatePerfFixtures/createTheme-complex.js`.

The pre-split baseline warned on **nine**, and only six of the eleven name the
same benchmark. Three warned then and not now — `ModuleWalk/imported/4x`,
`SeenModuleSource/kept/calls/4x` and `TransformDebugNamespacesPerCall/dev/32` —
and each of those got faster. Five warn now and did not then, and three of the
five also got faster, which a faster benchmark alone does not explain. Read the
two warning sets as a rough gauge of which fixtures sit near the sampling
limit, not as a measurement.

**Two benches measure nothing.** Both are in
`crates/stylex-css-parser/benches/properties_bench.rs` and both build
`Transform::new(vec![])`, which moves an empty `Vec` into a one-field struct and
allocates nothing. `PropertyCreation/transform_create` reads **801.74 ps**,
about two cycles. `PropertyDisplay/transform_display` then calls `to_string()`
on that empty value and reads **7.32 ns**, because every step of the `Display`
impl is empty as well. Their siblings, which build real values, read 13.9 ns and
24.8 ns, and 170.7 ns and 416.9 ns. Both now link the allocator and never ask it
for anything. Recorded as
[ticket 57](../issues/57-two-benches-measure-nothing.md) rather than fixed here,
because changing what a bench measures is a change to the series this record
opens.

**One bench is named for work it does not do.**
`InlineStyleToCssString/rtl_flippable_pairs`, in
`crates/stylex-css/benches/css_generation_bench.rs`, reads 91.6 ns against
154.6 ns for its left-to-right sibling. `inline_style_to_css_string` takes no
options, so no flipping happens in either case; the right-to-left set is faster
only because its keys are already in kebab case and skip the property-name
conversion the other set pays. Recorded as
[ticket 58](../issues/58-a-bench-named-for-work-it-does-not-do.md).

**Six of the fourteen bench files assert nothing.** That is 179 of the 268
measurements: every one discards the `Result` it built.
`guidelines/PERFORMANCE.md` requires a bench to assert what it measures, because
"a refusal, a deopt, a swallowed panic and a cache hit are all fast". The three
findings above are what that gap looks like when somebody finally reads the
numbers. Recorded as
[ticket 59](../issues/59-six-bench-files-assert-nothing.md).

## How to compare against this

```sh
cargo bench -p <package> --bench <target> -- --baseline mimalloc
```

Name the bench target. A bare `cargo bench -p <package>` also runs the crate's
lib test harness as a bench target, and that harness rejects criterion's own
options with "Unrecognized option". The package id uses underscores
(`stylex_transform`) where the directory uses hyphens.

**Criterion keys a saved baseline by group and benchmark id, not by crate.**
This workspace has one `target/criterion` at the worktree root, holding 44 group
directories; a bench that moves to another crate keeps its saved baseline as
long as its id does not change, and two crates that share a group and id would
silently collide. `baseline.md` and [`ticket-09.md`](./ticket-09.md) say the
identity is per crate. That is wrong, and it is worth knowing which way: what
closed the pre-split series for the three evaluator benches was a separate
target directory per leg, not the crate they moved to.

What does close a series is the target directory. Delete or rebuild it and the
saved baseline is gone, so keep the log and the summary in the tracker, as this
record does, and treat `target/criterion` as a convenience.
