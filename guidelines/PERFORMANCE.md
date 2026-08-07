# Performance Policy

Commands: [Scripts](./SCRIPTS.md).

## What blocks

Only release comparisons block: paired same-runner, same-process
`bench:revisions` + `bench:verdict` on every published target, plus
`bench:budget` on `x86_64-unknown-linux-gnu` once its ceilings are seeded
([Budget](#budget)). PR paired comparisons and the
`github-action-benchmark` history are advisory. Cross-run noise is ~16% on
hosted Linux and ~34% on `x86_64-apple-darwin`, too coarse for a 10-20%
regression.

## Subjects

- PR base: merge-base with `origin/develop`, built in an isolated worktree.
- Release base: the exact previous npm package, verified by resolved version
  and a real native load.
- Release candidate: the artifacts being published, never a rebuild.
- Every subject must produce non-zero rules for every fixture before timing, so
  a broken no-op subject cannot look fast. Never register a zero-rule fixture.

Gates read validated `*.v1.json` only, never `output.json`'s `extra` string.
Measurement jobs hold `contents: read`; the PR comment comes from a separate
default-branch `workflow_run` reporter that validates the artifact instead of
running PR code.

The reporter binds an artifact to its source run by run id, PR number and the
head SHA the run merged -- the only fields it can rederive from the
`workflow_run` event. The merge commit and the merge-base are recorded as
provenance, not asserted. Staleness is decided by the head SHA alone: a
comparison against `merge-base(develop, head)` stays valid when the base branch
moves, so dropping the report then would only hide a valid result.

## Verdict

Per fixture: median of per-round `candidate_p50 / base_p50`, bootstrapped with a
deterministic seed. Lower bound >= 1.10 warns; >= 1.20 flags and retries the
flagged fixtures once; a reproduced breach fails. Upper bound <= 0.50 warns (an
impossible improvement means a broken benchmark). Warnings never block.

Testing the lower bound makes the real trigger points ~1.11x (warn) and ~1.21x
(fail); the CI half-width is ~0.5-1% at 10 rounds. This under-reports rather
than blocking on noise. Do not "fix" it by comparing point estimates.

Do not use `p50 +/- tinybench.moe`: it is the standard error of the mean, not a
p50 interval. tinybench 6 has no p95; use the tested quantile helper over
retained samples, never display strings.

## Sampling

10 rounds, 300 ms standard time budget, 100 ms warmup, 2 heavy iterations,
10000 resamples, 95% one-sided, batch size 1. ~5.5 min per leg.

Resolution comes from the round count, which the bootstrap resamples; samples
inside one round only sharpen that round's median. **Buy sensitivity with
rounds, never with per-round time.** Fast fixtures get batching -- 0.10 ms to
0.30 ms must stay detectable -- never an absolute-delta exemption.

Measured on one machine at these values: 0/22 false positives same-vs-same,
18/20 warned on a 1.08-1.19x regression (the 2 misses were truly under 1.10),
20/20 flagged at 1.5-2.3x.

## Budget

While `benchmark/budget.json` is `pending-calibration` it holds no ceilings:
`bench:budget` reports `unseeded` and the leg passes. Once ceilings are seeded
from repeated clean runs (robust upper bound plus headroom), a breach fails the
leg and, through the publish job, blocks the release. Ceilings are valid only on
the canonical environment (`x86_64-unknown-linux-gnu`, Node 24.18.0, recorded
runner image); drift fails as recalibration rather than comparing.

Nothing may write this file automatically. A breach is fixed by optimization or
rollback. An increase needs a reviewed change stating old/new ceilings, repeated
measurements, cause and user impact, alternatives, and why rollback is not
appropriate. Decreases may ratchet in proven improvements.
