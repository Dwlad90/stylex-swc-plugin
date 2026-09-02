# Ticket 13 — bench A/B

The evaluator core moved from `stylex-transform` into `stylex-evaluator`, and
nineteen crates lost a `crate-type = ["cdylib", "rlib"]` that nothing linked.

> **Superseded in part by [ticket 16](./ticket-16.md).** The final A/B has since
> been run: ticket 13's move, with the build held constant, is **+0.37%** median
> with 48 of 52 measurements inside ±4%, so the move costs nothing. The `cdylib`
> finding below **did not reproduce**. Ticket 16 rebuilt the baseline commit four
> times and the `cdylib`-dropped tree three times; the bands overlap, and the two
> groups that separate move the *opposite* way to leg 2 below. Every leg here is
> a single build per configuration, which a control shows cannot resolve an
> effect under about 5 points. The nineteen `cdylib`s are still worth dropping,
> for a quarter off the workspace rebuild rather than for throughput.
>
> Ticket 16 also found the crate type that did matter: the addon's own unused
> `rlib`, which switched fat LTO off for the published `.node`.

**The final A/B is not in this file.** The `cdylib` change landed after the two
legs below were measured, so both legs describe a configuration the branch no
longer has. It is deferred by decision, not forgotten. What is recorded here is
the measurement that found the `cdylib` cost, because that measurement is the
reason the change is in the commit at all.

## Method

| Item     | Value                                                             |
| -------- | ----------------------------------------------------------------- |
| Machine  | Apple M1 Max, 10 cores, 64 GB, macOS 26.6.1                       |
| Profile  | `bench` (`lto = true`, `debug = true`)                            |
| Settings | `--sample-size 20 --warm-up-time 2 --measurement-time 4 --noplot` |
| Baseline | `f57526ebb` (ticket 12), own target dir, saved as `parent-clean`  |
| Test     | this ticket, own target dir, `--baseline parent-clean`             |
| Benches  | all five the moved code can reach; 52 measurements                 |

Each leg used its own `CARGO_TARGET_DIR`, and the criterion baselines were
copied between them rather than shared -- ticket 12 records what a shared
directory does to this measurement.

## Leg 1: the move, with the `cdylib` still in place

| Measure      | Value            |
| ------------ | ---------------- |
| Measurements | 52               |
| Median       | +0.91%           |
| Range        | −7.37% to +17.88% |
| Faster       | 20 of 52         |
| Within ±4%   | 36 of 52         |
| Above +4%    | 14 of 52         |

The median is inside the layout floor earlier tickets measured. **The median is
the wrong number to read here**, because the measurements split cleanly by
whether the bench can reach the moved code:

| Group                     | n   | Median  |
| ------------------------- | --- | ------- |
| `StructuralKeyFallback`   | 2   | +10.85% |
| `EvaluatePerfFixtures`    | 8   | +8.19%  |
| `StructuralKeyDepth`      | 4   | +6.57%  |
| `EvaluateDepth`           | 4   | +1.48%  |
| `FullPipeline`            | 6   | +1.08%  |
| `SeenModuleSource`        | 4   | −0.29%  |
| `EngineFoldRoundTrip`     | 9   | −0.41%  |
| `ConcatenationChain`      | 4   | −3.01%  |
| `ModuleWalk`              | 6   | −3.13%  |

And the control group -- the benches that cannot reach the moved code at all --
is flat to faster: `ModuleWalk/no-calls` −3.06% and −7.37%, `StateManager/new`
−0.64%, `FullPipeline/no-calls` +0.50% and +2.16%.

**This is the opposite of ticket 12's result, so ticket 12's argument does not
transfer.** There, the bench that could not reach the moved code shifted _more_
than the one that could, which ruled the moved functions out and left function
placement under fat LTO. Here the control is flat and the groups that reach the
moved code are up 6% to 18%. That is a cost in the move, not layout.

## Leg 2: the same move, `cdylib` dropped from the evaluator

Same baseline, same target directory, one line changed.

| Bench                              | with `cdylib` | `rlib` only | recovered |
| ---------------------------------- | ------------- | ----------- | --------- |
| `StructuralKeyFallback/object/128` | +17.88%       | +2.28%      | 15.6 pts  |
| `StructuralKeyDepth/arithmetic/60` | +7.11%        | +0.89%      | 6.2 pts   |
| `StructuralKeyDepth/arithmetic/120`| +6.03%        | −0.79%      | 6.8 pts   |
| `StructuralKeyDepth/arithmetic/240`| +7.85%        | +2.01%      | 5.9 pts   |
| `EvaluateDepth/arithmetic/240`     | +6.16%        | +1.91%      | 4.3 pts   |
| **Overall median (18 benches)**    | **+6.64%**    | **+2.14%**  | 4.5 pts   |

By group: `StructuralKeyDepth` +6.57% → **+0.14%**, `StructuralKeyFallback`
+10.85% → **+2.09%**.

`EvaluatePerfFixtures` is the exception: +8.19% → +7.38%, which the `cdylib`
does not explain. Its per-fixture numbers moved by up to 4 points between two
runs of identical code, so part of that is the group's own spread; whether a
real cost remains is for the deferred measurement to answer.

## Why the `cdylib` was the cause

`stylex-transform` has no `crate-type` line -- it is `rlib` only, and a comment
recorded that a `cdylib` broke the link there. So every line of the evaluator
was compiled `rlib` only for as long as it lived in the transform.
`stylex-evaluator` declared `crate-type = ["cdylib", "rlib"]`, as eighteen other
crates did. Moving 11k lines of the compiler's hottest code from an `rlib`-only
crate into a `cdylib` crate is the whole of the change those benches saw.

A `cdylib` exports its public symbols as preemptible, so a caller cannot
optimize into them, and Cargo cannot hand the crate's bitcode to the fat LTO the
`bench` and `release` profiles ask for. The effect scales with how much hot code
sits inside the `cdylib`, which is why a 210-line crate showed nothing and an
11k-line one showed 6% to 18%.

**Nothing linked any of those nineteen `cdylib`s.** Only
`crates/stylex-rs-compiler` needs one, for the `.node` Node loads. No crate here
builds as a WASM SWC plugin -- there is no `plugin_transform!` entry point in the
tree, and the `.cargo/config` the root `Cargo.toml` comments still mention no
longer exists. So all nineteen were dead output that also cost throughput, and
all nineteen are now `rlib` only.

**This is worth re-testing against the earlier tickets.** Ticket 07 measured a
+3.65% floor on a bench whose crate was byte-identical between legs, and ticket
12 a +3.04% median; both attributed the shift to function placement under fat
LTO. Both moves also went from the `rlib`-only transform into a `cdylib` crate,
so some of that floor may have been this. That is
[ticket 16](../issues/16-measure-the-crate-type-change.md).

**Ticket 16's answer: neither.** Both figures sit inside the noise of a single
build per leg, which a control measures at about ±2 points on the median and
±10 per measurement. The floor was the method.

Logs: `bench-13-parent-leg.log`, `bench-13-branch-leg.log`,
`bench-13-nocdylib-probe.log`.
