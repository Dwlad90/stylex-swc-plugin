# Ticket 16 — the crate-type change, measured

Three questions, and the answers are not the ones ticket 13 recorded.

1. Dropping the nineteen unlinked `cdylib`s has **no throughput effect this
   method can resolve**. Ticket 13's 4.5-point recovery does not reproduce.
2. Ticket 13's own code move, with the build held constant, is **+0.37%**.
3. The crate type that did matter is the one ticket 13 left alone. The addon
   carried an `rlib` that switched fat LTO **off** for the published `.node`.
   Removing it makes **64 of 64** benchmark fixtures faster, a median of
   **16.5%**.

## Method

| Item     | Value                                                             |
| -------- | ----------------------------------------------------------------- |
| Machine  | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1                       |
| Profile  | `bench` (`lto = true`, `debug = true`) for criterion              |
| Settings | `--sample-size 20 --warm-up-time 2 --measurement-time 4 --noplot` |
| Baseline | the `parent-clean` baselines ticket 13 saved on `f57526ebb`       |
| Benches  | the same five, 52 measurements per run                            |

One target directory per leg, never shared, criterion baselines copied in.
Scripts are in [`../tools/`](../tools/): `run-leg.sh` builds and measures a leg,
`summarize.py` reads a log, `bands.py` compares two configurations across their
replicate builds, `build-cost.sh` times a build, and `node-bench.sh` prices the
addon.

## The control, which the earlier tickets never took

**`f57526ebb` was rebuilt from scratch and measured against its own saved
baseline** — identical source, identical `Cargo.toml`, nothing changed. Four
such builds:

| Run                    | Median |
| ---------------------- | ------ |
| control 1              | −2.07% |
| control 1, re-measured | −2.00% |
| control 2              | −3.01% |
| control 3              | −2.02% |
| control 4              | −2.60% |

Two things follow, and both matter more than any single leg in this effort.

**The stored `parent-clean` baseline carries a systematic −2% to −3% drift.**
Identical code reads 2 to 3 points faster than the day the baseline was taken,
and `ModuleWalk` reads 7 to 9 points faster. Any future comparison against those
saved baselines inherits that offset. Compare same-day legs, or re-take the
baseline.

**A single measurement carries about ±10 points.** Re-measuring the *same
binary* moved `ModuleWalk` from −7.50% to −9.38%, and
`EvaluatePerfFixtures/dynamic-param-shadows-import-edges` read +12.85% against
identical code. Per-*group* medians are far steadier than per-measurement ones,
which is why every judgement below is a group median across replicate builds.

## Question 1: the crate-type change on its own

`f57526ebb` with the nineteen `cdylib`s dropped and nothing else changed, three
independent builds, against the four control builds.

| Configuration        | n | Whole-suite median band |
| -------------------- | - | ----------------------- |
| `cdylib` kept        | 4 | −3.01% to −2.02%        |
| `cdylib` dropped     | 3 | −2.71% to −1.28%        |

The bands overlap, so there is **no whole-suite effect**. Ten of twelve groups
overlap too. Two do not, and they move the *opposite* way to ticket 13:

| Group                   | `cdylib` kept (n=4) | dropped (n=3)    | verdict            |
| ----------------------- | ------------------- | ---------------- | ------------------ |
| `StructuralKeyDepth`    | −2.44% to −1.95%    | −0.14% to +1.45% | slower by >=1.8 pts |
| `StructuralKeyFallback` | −1.57% to −0.44%    | +0.61% to +2.23% | slower by >=1.0 pts |

**Ticket 13's leg 2 is not reproducible.** It read a single build each way and
recorded `StructuralKeyDepth` +6.57% → +0.14% and `StructuralKeyFallback`
+10.85% → +2.09%, a 4.5-point median recovery. Here, across seven builds, the
sign is reversed and the size is a third of that. The mechanism it proposed also
does not fit: at `f57526ebb` the evaluator core still sits in `stylex-transform`,
which was **already `rlib` only**, so symbol preemptibility in *other* crates
cannot reach those functions. What moves these two groups is codegen placement
inside one fat-LTO unit, and its sign changes from build to build.

## Question 2: ticket 13's move, with the build held constant

Both legs read against the same baseline, so the ratio of one to the other
follows from the two:

| Measure      | Value             |
| ------------ | ----------------- |
| Measurements | 52                |
| Median       | **+0.37%**        |
| Range        | −7.48% to +5.78%  |
| Within ±4%   | 48 of 52          |

`StructuralKey/call/shallow` benchmarks `stylex-utils`, byte-identical between
these two legs, and reads **−0.05%**. `EvaluatePerfFixtures` reads **+0.69%**,
against the +8.19% ticket 13 recorded — that loose end is **spread, not a cost**,
and no bench group shows a real cost from the move.

## Question 3: the addon, and the LTO that never ran

`profile.release` asks for `lto = true`, and its comment claimed fat LTO enables
cross-crate inlining into SWC's visitor traversal. It did not.

**LTO reaches only a final artifact.** Cargo works out LTO per unit, and a unit
whose target is a linkable library -- `rlib`, `lib` or `dylib` -- gets no LTO,
because one rustc invocation cannot both LTO-link a final artifact and emit a
reusable library. This is deliberate, and cargo prints no warning. So the addon,
which declared `["cdylib", "rlib"]`, was never a final artifact and never got
LTO. From two verbose builds of `stylex-rs-compiler`:

| Config                             | LTO flags rustc receives                            |
| ---------------------------------- | --------------------------------------------------- |
| `["cdylib", "rlib"]` — as shipped  | **none**; the word `lto` appears nowhere in the build |
| `["cdylib"]`                       | `-C lto`, plus 212 dependencies with `-C linker-plugin-lto` |

Nothing links that `rlib`: no `Cargo.toml` in the workspace depends on
`stylex_compiler_rs`, and the crate has no `tests/` or `benches/` directory. Its
82 unit tests pass without it.

Priced through the published entry point with `pnpm bench`, two runs per
configuration, alternating (a NAPI addon cannot be swapped inside one process on
macOS, so `bench:revisions` cannot run here):

| Measure                          | Value              |
| -------------------------------- | ------------------ |
| Fixtures faster                  | **64 of 64**       |
| Median change                    | **−16.48%**        |
| Mean change                      | −18.23%            |
| Best                             | `calls that do not fold` −38.68% |
| Decided by non-overlapping runs  | 64 of 64           |

Every fixture, no overlap anywhere. This is what a real effect looks like next
to the noise in questions 1 and 2.

**What this measurement cannot say.** There are two runs per configuration, not
more, so the −16.48% is a direction and a rough size, not a precise figure. The
two addons also ran in separate processes, because a NAPI addon cannot be
swapped inside one process on macOS, so anything that varies between processes
-- allocator warm-up, page cache, thermal state -- sits inside the effect rather
than beside it. Alternating the configurations limits that; it does not remove
it. The direction is safe because all 64 fixtures agree and no run overlaps,
which is far outside the +-10 point spread the controls measure.

The panic transport still works with LTO on: an unbound `stylex.create()` throws
a catchable `Error` carrying the diagnostic, and the process survives. That
check mattered because `profile.release` rejects `panic = "abort"` for exactly
that transport, and the note recording it was written against a build with no
LTO.

Logs: [`ticket-16-nodebench/`](./ticket-16-nodebench/), and the flag evidence in
[`ticket-16-lto-flags.txt`](./ticket-16-lto-flags.txt).

## Build cost and artifact size

Identical source, only `Cargo.toml` differing. The **workspace-only rebuild**
cleans this workspace's crates and leaves every dependency built, so the
crate-type change is the whole of the difference; the cold build is dominated by
400-odd third-party crates and shows nothing.

| Measure                       | `cdylib` kept   | `cdylib` dropped |
| ----------------------------- | --------------- | ---------------- |
| Workspace rebuild, `dev`      | 10.61 / 10.77 s | **8.02 / 8.21 s** |
| Workspace rebuild, `release`  | 63.39 s         | **47.47 s**      |
| Cold build, `dev`             | 113.11 / 111.04 s | 112.73 / 110.79 s |
| Cold build, `release`         | 147.65 s        | 149.80 s         |
| Dynamic libraries emitted     | 20              | **1**            |
| Target size, `dev`            | 3095 MiB        | 3088 MiB         |

**About a quarter off the rebuild a developer pays on every edit**, in both
profiles, with a within-configuration spread of 0.16 s against a 2.4 s gap. This
is the reason the nineteen `cdylib`s should stay gone. It is not the reason
ticket 13 gave.

The addon's `rlib` costs the other way:

| Measure                      | `["cdylib", "rlib"]` | `["cdylib"]` |
| ---------------------------- | -------------------- | ------------ |
| Cold build, `release`        | 144.79 s             | 248.52 s     |
| Workspace rebuild, `release` | 39.25 s              | 151.78 s     |
| Published `.node`            | 18,724,368 B         | 19,538,192 B |

The 4x build time and the 4.3% larger addon are the cost of the fat LTO finally
running. For the one crate that ships, throughput wins.

## `crate-type = ["rlib"]` against no `crate-type` line

The ticket asked whether the crates should drop the line entirely, as
`stylex-transform` did before this effort. They are equivalent — `lib` resolves
to `rlib` — and the measurement agrees:

| Measure                      | explicit `["rlib"]` | no line   |
| ---------------------------- | ------------------- | --------- |
| Workspace rebuild, `dev`     | 8.65 s              | 8.00 s    |
| Workspace rebuild, `release` | 43.23 s             | 42.37 s   |
| Addon `.dylib`               | 18,288 KiB          | 18,288 KiB |

The line stays, because it states the intent that ticket 13's comment could only
assert, and because an accident is what left `stylex-transform` untyped in the
first place.

## What this says about the method

Every conclusion in tickets 07, 12 and 13 rests on one build per leg. A control
shows one build per leg cannot resolve anything under about 5 points per group.
Two rules for any later bench in this effort:

- **Take a control.** Rebuild the baseline commit and measure it against its own
  saved baseline. Whatever it reads is the floor of what the run can claim.
- **Replicate the build, not just the measurement.** Re-running a binary is
  cheap and hides the variance that matters, which enters at link time under
  `-C lto -C codegen-units=1`.
