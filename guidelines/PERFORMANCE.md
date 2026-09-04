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

## The allocator a bench measures

The published addon runs mimalloc on all seven targets. `swc_malloc` chooses it
for the six that are not musl, and the addon names it directly for
`x86_64-unknown-linux-musl`, because `swc_malloc` declines every musl target at
once.

A criterion bench measures the system allocator unless it links the same crate,
and Rust links a dev-dependency only where a target names it. A manifest entry
alone therefore does nothing: the line that makes the choice real is
`use swc_malloc as _;` in the bench file.

Every bench in the workspace carries that line.
`every_bench_says_which_allocator_it_measures`, in the addon's own test module,
fails when a new bench carries neither that line nor an `ALLOCATOR: system`
note giving the reason.

One gap stays, and it is a small one. A bench built for musl still measures the
system allocator, because `swc_malloc` declines musl and only the addon names
mimalloc for that target. No bench runs there today.

**Numbers taken before this rule do not compare with numbers taken after it.**
The allocator is the axis several benches sit on, so a series that crosses the
change reads a difference that no code change caused. Re-baseline first, then
compare.

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

## Writing a bench

**A bench that touches the transform must run inside `GLOBALS.set`.**
`parse_and_normalize_program` and `StyleXTransformBuilder::into_pass` both call
`Mark::new()`, which panics outside a `GLOBALS` scope. The panic does not
surface: the code frame is a diagnostic aid behind a panic boundary, so the
bench still reports a number -- it times a panic and its unwind instead of the
work, and reports a regression in the swallowed path as an improvement. That
mistake inflated one attribution of the debug path by 3.6x before it was
caught. Set it once around the whole benchmark function, as the benches under
`crates/stylex-transform` and `crates/stylex-evaluator` do.

Assert what the bench is measuring, in the bench. A refusal, a deopt, a
swallowed panic and a cache hit are all fast, and a curve that flattens because
the work stopped happening is indistinguishable from a win. Every bench in
`crates/stylex-transform/benches` and `crates/stylex-evaluator/benches` panics
unless its subject produced the output it exists to time -- a fold that reached
the expected value, a `dev` transform that resolved one `file:line` per style.

Both configurations are worth watching, and they are watched separately. `dev`
implies `debug`, and `debug` turns on the `file:line` annotation on `$$css`,
which costs several times the whole production transform; the two cannot be
compared against each other. `benchmark/lib/config.ts` therefore keeps
`dev: false` as the shared shape, and a fixture opts into the other one with
`"dev": true` in `benchmark/fixtures.v1.json`. Never flip the shared option: it
moves every trend series in the repo at once.

Every other development or compatibility feature is priced the same way, through
an `"options"` map on the fixture that asks for it -- the debug data prop and
debug class names, unminified keys, reading the source off disk, legacy
shorthand expansion, the logical-property polyfill and RTL comments,
`px`-to-`rem`, media query ordering, and the two enum-valued options. The keys
are an allowlist in `benchmark/lib/types.ts`; a manifest naming anything else
fails to load rather than being measured under the shared shape while claiming
otherwise.

**Every key in a fixture's option map must change what the compiler emits.** The
per-entry check below is not enough: an entry passes it as soon as _one_ key
moves the output, which let an entry named for a chained input source map price
`dev: true` while carrying a map that made no difference at all — nor did a
garbage one. `fixtures.test.ts` therefore varies one key at a time.

A boolean key is **flipped**, not dropped, and that is the whole of what the
check asks. Dropping would reject a key that restates a default —
`enableDebugDataProp` is already on under `debug`, `useRealFileForSource` under
`dev` — and an entry is entitled to name the option it exists to measure rather
than leave a reader to know the defaults. What must not pass is an option the
compiler does not react to at all, which is what flipping catches. A key whose
value is not a boolean has no flip and is dropped instead, which is how the
inert input source map was found.

Of `enableLogicalStylesPolyfill`, `enableLegacyValueFlipping` and
`enableLTRRTLComments`, only the middle one changes anything on the RTL fixture,
so it is the only one registered. The chained-input-map path is left unmeasured
rather than faked: a map generated from the fixture itself maps to the same
positions, so the transform emits the same module, and pricing it needs a map
from an earlier tool that really moved the code.

**A fixture's options must change what the compiler emits.** `fixtures.test.ts`
fails an entry whose emitted module, metadata and source map are identical to
its production run, and that test found seven entries measuring nothing:
`enableMediaQueryOrder`, `legacyDisableLayers`, `propertyValidationMode: throw`
and `treeshakeCompensation: false` changed not one byte on any fixture in the
corpus; `sourceMap: True` changed nothing, because this compiler emits a map in
its production shape already; a `(dev)` twin of a token file emitted the same
module as its production run; and `enableFontSizePxToRem` was pointed at a
fixture whose only font size was already in `rem`. Each of them reported a
development feature and measured the production shape. The allowlist in
`benchmark/lib/types.ts` therefore holds only keys a fixture uses, and
`aliases`, `definedStylexCssVariables` and `importSources` are absent for the
same reason — the last one because it made the transform _faster_ by emitting
less, which is the "fast because the work stopped happening" trap this file
opens with.

**The data prop is emitted where styles are read.** `data-style-src` needs a
`stylex.props` or `stylex.attrs` call site; a fixture that only calls `create`
cannot measure it however many debug options it names.
`perf_fixtures/props-and-attrs.js` is that call site, and a test asserts the
entries named for the data prop actually emit one.

**A fixture must compile on the merge base, too.**
`bench:revisions` runs the manifest against two subjects -- this branch's build
and one built from the merge base -- and sanity-checks every fixture on both
before timing anything. A shape that only a fix makes compilable is a
correctness question, so it belongs to `crates/stylex-transform/tests/fixture`
and stays there; `perf_fixtures/dynamic-styles.js` states this rule in its own
header and leaves that shape out while still pricing the inline-style path.
`selectMeasurableFixtures` names the fixture and the subject that refused it, so
the next one reads as the manifest question it is rather than as a bare compiler
stack.

**The release leg reads a base refusal differently, and says so on the command
line.** The two legs that pair subjects do not share a base. The pull-request
leg builds the merge base, where the rule above holds and every refusal stops
the run. The release leg installs the _last published version_, which is behind
this build by every feature landed since, so a fixture that prices one of those
features has no second side to compare against -- one `.trim()` in
`perf_fixtures/engine-fold.js` cost the whole publish benchmark that way. That
leg passes `--allow-base-refusals`: such a fixture is reported under
`Not compared`, written into the raw stats beside the numbers, and left out of
the run, and it returns on its own once the published baseline carries the
feature. The flag is off by default, so the strict reading is what a caller gets
without asking. The gate the flag never lifts is the candidate: a fixture _it_
refuses, or compiles to no rules, is a regression in the code under measurement
and fails the leg. A run where no fixture survives fails as well, since a base
that refuses everything is a broken subject rather than a manifest question.

**Register a feature fixture in pairs.** One number for a development shape says
nothing about what the feature costs; the pair does. A `Feature - x` entry and a
`Feature - x (dev)` entry point at the same file and differ only in the option
map, and `fixtures.test.ts` fails a `(dev)` entry with no production twin. Size
is not a feature: `apps/rollup-large-example/lotsOfStyles.js` is one `create`
call repeated thousands of times, so it prices throughput and nothing else. A
new fixture earns its place by exercising a capability none of the others
reach.

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
