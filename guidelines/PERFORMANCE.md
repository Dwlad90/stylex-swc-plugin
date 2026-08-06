# Performance Policy

Commands: [Scripts](./SCRIPTS.md).

## What decides a regression

Only paired same-runner, same-process comparisons block: `bench:revisions` +
`bench:verdict` on every non-draft PR and on every runnable published target
before release, plus `bench:budget` against absolute p95 ceilings on
`x86_64-unknown-linux-gnu`. The `github-action-benchmark` history is reporting
only -- cross-run noise is ~16% on hosted Linux and ~34% on
`x86_64-apple-darwin`, which cannot resolve a 10-20% regression.

Subjects must be exactly what they claim: PR base is the merge base with
`origin/develop` built in an isolated worktree; release base is the exact
previous npm package, verified by resolved version and a real native load;
release candidates are the artifacts that get published, not a rebuild. Every
subject must transform every fixture with valid metadata and non-zero rules
before timing, so a broken no-op subject cannot look fast.

Gates read only validated versioned JSON (`*.v1.json`), never the
human-readable `extra` in `output.json`. Measurement jobs hold `contents: read`;
the PR comment comes from a separate default-branch `workflow_run` reporter that
validates the artifact instead of running PR code.

## Verdict

Per fixture: median of per-round `candidate_p50 / base_p50`, bootstrapped with a
deterministic logged seed. Lower bound >= 1.10 warns; >= 1.20 flags and retries
once; a reproduced breach fails the suite. Upper bound <= 0.50 warns -- an
impossible improvement usually means a broken benchmark. Warnings never block.

Do not use `p50 +/- tinybench.moe`: it is the standard error of the mean, not a
p50 interval. tinybench 6 has no p95; use the tested quantile helper over
retained samples, never display strings.

## Calibration status: not yet calibrated

Current defaults (10 rounds, 10000 resamples, 95% one-sided, warn 1.10, fail
1.20, batch size 1) are starting values, not measurements. Before treating them
as calibrated, archive >=10 same-vs-same hosted-runner runs with zero
suite-level false failures (judged over the whole 23-fixture suite) and injected
10%/20%/larger slowdowns showing 20% blocks reliably. Fast fixtures get batching
-- 0.10 ms to 0.30 ms must stay detectable -- never an absolute-delta exemption.
Record the resulting parameters and evidence here.

## Budget

`benchmark/budget.json` is `pending-calibration` and reports without blocking
until ceilings are seeded from repeated clean runs using a documented robust
upper bound plus headroom. Ceilings are valid only on the canonical environment
(`x86_64-unknown-linux-gnu`, Node 24.18.0, recorded runner image); drift fails as
recalibration rather than comparing. Nothing may write the file automatically: a
breach is fixed by optimization or rollback, an increase needs a reviewed change
stating old/new ceilings, repeated measurements, cause and user impact,
alternatives, and why rollback is not appropriate. Decreases may ratchet in
proven improvements.
