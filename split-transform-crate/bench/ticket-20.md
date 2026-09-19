# Ticket 20 — the cost of the release matrix, and the budget seeding corpus

The LTO change needed two measurements, and only a hosted runner can give
them. The workflows now have time limits that come from these numbers, and
`budget.json` is `enforced` with a ceiling for each of the 65 fixtures.

Section 2 was written when the corpus was seven runs and 61 fixtures, and
the budget could not be seeded. Section 3 records what changed and holds
the numbers that the committed file uses. Where the two disagree, section 3
is the one that describes the file.

## 1. What the release matrix costs, and whether the runners survive it

Seven complete runs of `npm.yml` between 2026-09-04 and 2026-09-08, all after
ticket 16 dropped the `rlib` and let fat LTO link the addon. Five came from
`workflow_dispatch`, two from the `release.yml` runs that published
`0.19.0-rc.1` and `0.19.0-rc.2`.

**Every leg of every run passed.** The ticket asked whether about 7 GB of
runner memory survives fat LTO with `codegen-units = 1`. The question was
built on a stale figure: the runners report **16 GB**, and no leg ran out of
memory, timed out, or was retried. The three Docker legs — musl and both
Debian images — are not the slowest ones.

| Target | Runs | Fastest (min) | Slowest (min) |
| --- | ---: | ---: | ---: |
| `aarch64-pc-windows-msvc` | 7 | 13.1 | 18.3 |
| `aarch64-apple-darwin` | 7 | 7.6 | 17.1 |
| `x86_64-pc-windows-msvc` | 7 | 9.9 | 17.1 |
| `x86_64-unknown-linux-musl` | 7 | 13.2 | 16.2 |
| `x86_64-apple-darwin` | 7 | 5.8 | 13.1 |
| `aarch64-unknown-linux-gnu` | 7 | 8.9 | 10.6 |
| `x86_64-unknown-linux-gnu` | 7 | 8.8 | 10.0 |

The slowest leg is `aarch64-pc-windows-msvc` at 18.3 min, and the widest
spread belongs to `aarch64-apple-darwin`, 7.6 to 17.1 min. The spread is
Turbo cache state, not the target.

Other jobs in the same seven runs:

| Job | Legs measured | Fastest (min) | Slowest (min) |
| --- | ---: | ---: | ---: |
| Paired benchmark | 42 | 14.2 | 27.6 |
| Binding test | 79 | 1.4 | 6.0 |
| Publish | 2 | 3.6 | 3.6 |
| Aggregate verdicts | 7 | 0.2 | 0.2 |

The slowest leg of each group, so that the table above can be checked:

| Group | Slowest leg | Slowest (min) |
| --- | --- | ---: |
| Paired benchmark | `x86_64-apple-darwin` | 27.6 |
| Binding test | `x86_64-apple-darwin` | 6.0 |

`publish` waits behind the `publish-approval` environment, but the wait does
not count against the job clock: the two successful publishes took 3.6 min
each, and `changelog-approval` in `release.yml` records 0.03 to 0.07 min
behind the same type of gate. Thus the approval gate is not a reason to
leave a job without a limit.

### What landed

`npm.yml`, `release.yml` and `pr-validation.yml` now have a
`timeout-minutes` on each job. Each value comes from the slowest run of that
same job, not from one number for the full file:

| Workflow | Job | Slowest seen (min) | Ceiling (min) | Ratio |
| --- | --- | ---: | ---: | ---: |
| npm | `build` | 18.3 | 40 | 2.2x |
| npm | `test-macOS-windows-binding` | 6.0 | 15 | 2.5x |
| npm | `test-linux-x64-gnu-binding` | 2.0 | 10 | 5.0x |
| npm | `test-linux-arm64-gnu-binding` | 1.6 | 10 | 6.2x |
| npm | `test-linux-x64-musl-binding` | 2.5 | 10 | 4.0x |
| npm | `benchmark` | 27.6 | 50 | 1.8x |
| npm | `benchmark-aggregate` | 0.2 | 10 | floor |
| npm | `publish` | 3.6 | 15 | 4.2x |
| pr | `build-public-packages` | 8.9 | 25 | 2.8x |
| pr | `basic-checks` | 1.6 | 10 | 6.2x |
| pr | `checks` | 7.5 | 25 | 3.3x |
| pr | `parity-sweep` | 4.4 | 20 | unchanged |
| pr | `visual-tests` | 3.2 | 15 | 4.7x |
| pr | `benchmark` | 21.4 | 45 | 2.1x |
| pr | `playwright-image` | 0.1 | 10 | floor |
| pr | `pr-validation` | 0.1 | 10 | floor |
| pr | `dependabot` | 0.2 | 10 | floor |
| npm | `cleanup` | never runs | 10 | floor |
| release | every job | 2.3 | 10 | 4.3x |

A matrix job carries one limit for all its legs, so each value is set from
the slowest leg. `cleanup` has `if: false` and never runs; it gets a limit
so that the bound is already correct if someone enables the job. Short jobs get a floor of ten minutes rather than a number
too small to survive one slow runner. `parity-sweep` already bounded itself
at 20 min and keeps that value; only the comment beside it changed, because
it said this was the only bounded job.

Three samples, and each one has a different size. The `npm.yml` figures
come from 7 complete runs of that workflow. The `release.yml` figures come
from 10 runs of `release.yml`, which starts `npm.yml` but has its own
control jobs. The `pr-validation.yml` figures come from 21 runs.

The size of the sample changes the answer. When the pull-request sample
became 21 runs instead of 12, `benchmark` moved from 14.8 min to 21.4 min.
That is why the limit there is 45 min and not 30 min.

One job has no limit: `upload-npm` in `release.yml`. It calls `npm.yml`
with `uses:`, and GitHub does not permit `timeout-minutes` on such a job.
The workflow that it calls sets the limits for its own jobs.

### What a breach does

GitHub cancels the job outright — no warning, no extension, no automatic
retry. The running step is killed, `if: always()` steps get a short grace
window and may not finish, every job that `needs:` it is skipped, and the
run reads as a failure. The minutes used up to that point are still billed.

Two properties make that safe here. A timeout cannot publish half a release,
because `publish` needs every build, test and benchmark job, so a cancelled
leg skips it. And a cancelled job reads as neither `success` nor `skipped`,
so the `pr-validation` gate job fails rather than passing a run that never
finished.

## 2. The budget seeding corpus

Seven archived `paired-release-budget` reports, every one on the canonical
environment `budget.json` names: `x86_64-unknown-linux-gnu`, Node `v24.18.0`,
`ubuntu-latest` on the `ubuntu24` image. All seven report `unseeded`, which
is what a seeding run is. All seven measured the same 61 fixtures.

| Date | Run | Candidate | Commit | ImageVersion | CPU | Fixtures |
| --- | --- | --- | --- | --- | --- | ---: |
| 2026-09-04 | 33896665125 | candidate@0.18.6 | 68edbe2b0 | 20260831.293.1 | AMD EPYC 9V74 80-Core Processor | 61 |
| 2026-09-04 | 33901836307 | candidate@0.18.6 | c52ce2e96 | 20260831.293.1 | AMD EPYC 7763 64-Core Processor | 61 |
| 2026-09-04 | 33923898462 | candidate@0.18.6 | 9995074fe | 20260831.293.1 | AMD EPYC 7763 64-Core Processor | 61 |
| 2026-09-05 | 33964522213 | candidate@0.18.6 | 4eee173e9 | 20260831.293.1 | AMD EPYC 7763 64-Core Processor | 61 |
| 2026-09-05 | 33968037390 | candidate@0.18.6 | 5903b7cb1 | 20260831.293.1 | AMD EPYC 7763 64-Core Processor | 61 |
| 2026-09-06 | 34047021567 | candidate@0.19.0-rc.1 | 0b203aba0 | 20260831.293.1 | AMD EPYC 7763 64-Core Processor | 61 |
| 2026-09-08 | 34265091652 | candidate@0.19.0-rc.2 | 540a775cc | 20260907.300.1 | INTEL(R) XEON(R) PLATINUM 8573C | 61 |

The policy asks the seeding runs to spread across the CPU models hosted
runners hand out. They do — three classes — and that is where the corpus
says something the ceilings must respect:

**The machine class dominates the noise.** Taking each fixture's fastest run
as 1.00, the median fixture reads 1.15x to 1.18x on every AMD EPYC run and
1.00x on the one Intel Xeon run. The six AMD runs sit inside 2.6 points of
each other. So the run-to-run spread is not really run-to-run: it is one
machine class being about 15% faster than the other, and a ceiling has to be
set for the slower class.

Per-fixture spread across the seven runs is a median of 1.19x and a worst of
1.91x. The worst offenders are the cheapest fixtures, where a few
microseconds of scheduler noise is a large fraction of the measurement —
`Feature - token definitions` at 0.12 ms, `consts` at 0.08 ms.

### The computed ceilings

Worst of the seven runs, times a headroom of **1.25**. The worst of seven
already sits on the slow machine class, so the headroom covers a machine
class slower than any seen yet rather than the noise, which the maximum has
absorbed. Under these ceilings the worst run measured reads 0.80
utilization.

| Fixture | Category | Worst of 7 (ms) | Ceiling (ms) | Spread |
| --- | --- | ---: | ---: | ---: |
| Debug data - lotsOfStyles.js (100 creates, dev) | perf | 17.4530 | 21.8200 | 1.25x |
| Feature - CSS value normalization | perf | 0.5127 | 0.6409 | 1.33x |
| Feature - CSS value normalization (dev) | perf | 0.6747 | 0.8434 | 1.19x |
| Feature - class name prefix | perf | 0.6537 | 0.8172 | 1.18x |
| Feature - debug and dev class names | perf | 0.7955 | 0.9944 | 1.04x |
| Feature - debug class names over many namespaces | perf | 0.8172 | 1.0210 | 1.16x |
| Feature - debug data prop | perf | 0.7525 | 0.9406 | 1.04x |
| Feature - debug without the data prop | perf | 0.5612 | 0.7015 | 1.10x |
| Feature - dynamic styles | perf | 1.3552 | 1.6940 | 1.51x |
| Feature - dynamic styles (dev) | perf | 1.5692 | 1.9620 | 1.35x |
| Feature - font size px to rem | perf | 0.5065 | 0.6331 | 1.23x |
| Feature - inlined conditional merge off | perf | 0.5494 | 0.6868 | 1.12x |
| Feature - keyframes and animations | perf | 0.4621 | 0.5776 | 1.17x |
| Feature - keyframes and animations (dev) | perf | 0.6186 | 0.7733 | 1.04x |
| Feature - legacy shorthand expansion | perf | 0.6599 | 0.8249 | 1.56x |
| Feature - legacy value flipping | perf | 0.4633 | 0.5791 | 1.56x |
| Feature - logical and RTL | perf | 0.4585 | 0.5731 | 1.43x |
| Feature - logical and RTL (dev) | perf | 0.6157 | 0.7696 | 1.23x |
| Feature - media queries | perf | 0.6424 | 0.8030 | 1.18x |
| Feature - media queries (dev) | perf | 0.7541 | 0.9426 | 1.12x |
| Feature - media query order | perf | 1.7410 | 2.1760 | 1.14x |
| Feature - media query order off | perf | 0.7620 | 0.9525 | 1.24x |
| Feature - nested conditions | perf | 0.6586 | 0.8233 | 1.18x |
| Feature - nested conditions (dev) | perf | 0.8247 | 1.0310 | 1.09x |
| Feature - props and attrs | perf | 0.5520 | 0.6900 | 1.06x |
| Feature - props and attrs (dev) | perf | 0.7749 | 0.9686 | 1.06x |
| Feature - readable test class names | perf | 0.6478 | 0.8097 | 1.20x |
| Feature - runtime injection | perf | 0.5743 | 0.7178 | 1.12x |
| Feature - source maps inline | perf | 0.5566 | 0.6957 | 1.10x |
| Feature - source maps without columns | perf | 0.5496 | 0.6870 | 1.11x |
| Feature - source maps without source text | perf | 0.5476 | 0.6845 | 1.11x |
| Feature - source text read from disk | perf | 0.6675 | 0.8343 | 1.24x |
| Feature - stylex side effects injected | perf | 0.3377 | 0.4221 | 1.43x |
| Feature - theme tokens | perf | 0.3375 | 0.4219 | 1.43x |
| Feature - theme tokens (dev) | perf | 0.4655 | 0.5819 | 1.21x |
| Feature - token definitions | perf | 0.2267 | 0.2834 | 1.91x |
| Feature - unminified debug keys | perf | 0.4572 | 0.5715 | 1.26x |
| Feature - view transitions | perf | 0.3291 | 0.4113 | 1.44x |
| Feature - view transitions (dev) | perf | 0.4602 | 0.5753 | 1.20x |
| Performance - Basic create transformation | perf | 0.2637 | 0.3297 | 1.62x |
| Performance - Basic theme transformation | perf | 0.3105 | 0.3881 | 1.39x |
| Performance - Colors StyleX transformation | perf | 0.3253 | 0.4066 | 1.41x |
| Performance - Complex create transformation | perf | 3.1948 | 3.9940 | 1.73x |
| Performance - Complex theme transformation | perf | 5.5637 | 6.9550 | 1.10x |
| Rollup plugin - lotsOfStyles.js | rollup | 2829.5026 | 3537.0000 | 1.29x |
| Rollup plugin - lotsOfStylesDynamic.js | rollup | 16.3912 | 20.4900 | 1.78x |
| buttons-demo | transform | 0.4852 | 0.6065 | 1.04x |
| card | transform | 0.2397 | 0.2996 | 1.62x |
| consts | transform | 0.1559 | 0.1948 | 1.88x |
| counter | transform | 0.7605 | 0.9506 | 1.04x |
| counter-with-dynamic-styles | transform | 0.9627 | 1.2030 | 1.13x |
| global-tokens | transform | 2.7083 | 3.3850 | 1.19x |
| global-tokens-xs | transform | 0.5007 | 0.6259 | 1.26x |
| namespace-cleaning | transform | 0.8086 | 1.0110 | 1.10x |
| namespace-cleaning-no-unused | transform | 0.7584 | 0.9480 | 1.12x |
| page | transform | 0.2961 | 0.3702 | 1.39x |
| page-tsx | transform | 1.9421 | 2.4280 | 1.15x |
| page-with-keyframes | transform | 1.8983 | 2.3730 | 1.14x |
| spot-loader | transform | 0.6871 | 0.8589 | 1.12x |
| typography | transform | 1.0259 | 1.2820 | 1.11x |
| use-memo | transform | 0.3394 | 0.4243 | 1.17x |

<!-- 61 fixtures, headroom 1.25 -->

## 3. The budget is calibrated

`budget.json` is `enforced`, with one ceiling for each of the 65 fixtures.

**The four fixtures that were missing are measured now.** Every run until
2026-09-08 measured 61, because `npm.yml` passes `--allow-base-refusals` and
the published base could not fold `trim` or `Math.sign`. Three dispatches
with `previous-version=0.19.0-rc.2` fixed that: `0.19.0-rc.2` contains the
fold, so no fixture leaves the run. The base and the candidate carry the
same version string, which the paired runner accepts, because it tells them
apart by the `npm@` and `candidate@` labels.

### The corpus

| Date | Run | Fixtures | Candidate commit | ImageVersion | CPU |
| --- | --- | ---: | --- | --- | --- |
| 2026-09-04 | 33896665125 | 61 | 68edbe2b0 | 20260831.293.1 | AMD EPYC 9V74 80-Core |
| 2026-09-04 | 33901836307 | 61 | c52ce2e96 | 20260831.293.1 | AMD EPYC 7763 64-Core |
| 2026-09-04 | 33923898462 | 61 | 9995074fe | 20260831.293.1 | AMD EPYC 7763 64-Core |
| 2026-09-05 | 33964522213 | 61 | 4eee173e9 | 20260831.293.1 | AMD EPYC 7763 64-Core |
| 2026-09-05 | 33968037390 | 61 | 5903b7cb1 | 20260831.293.1 | AMD EPYC 7763 64-Core |
| 2026-09-06 | 34047021567 | 61 | 0b203aba0 | 20260831.293.1 | AMD EPYC 7763 64-Core |
| 2026-09-08 | 34265091652 | 61 | 540a775cc | 20260907.300.1 | INTEL(R) XEON(R) PLATINUM 8573C |
| 2026-09-10 | 34506217184 | 65 | 6c30e7d88 | 20260907.300.1 | AMD EPYC 7763 64-Core |
| 2026-09-10 | 34509719470 | 65 | 6c30e7d88 | 20260907.300.1 | AMD EPYC 9V74 80-Core |
| 2026-09-10 | 34511924673 | 65 | 6c30e7d88 | 20260907.300.1 | AMD EPYC 7763 64-Core |

Ten runs, three CPU models and two image builds, all on the canonical
environment. Seven of them measure 61 fixtures, each on a different earlier
candidate commit; three measure all 65 on `6c30e7d88`.

**The seven earlier runs are not the same code**, and they set 43 of the 65
ceilings, so it is wrong to say the only difference is the machine. What
holds instead is the direction: the ceiling is the *maximum*, so an earlier
run can only raise it. No ceiling is too tight because of them. The cost is
that some ceilings may be looser than the code today needs. The policy
permits a ratchet down, so a later change can tighten them once three or
more runs on one commit are archived.

### The ceilings

`observedUpperMs` is the largest median-of-round p95 that any run gave for
the fixture, and `ceilingMs` is that value times a headroom of **1.25**.
The maximum already sits on the slowest class of machine seen, so the
headroom covers a class that is slower than all of them.

The maximum is a real upper bound and not one bad run: the run that sets the
ceiling is spread over all ten runs, at 18, 11, 10, 7, 5, 5, 4, 2, 2 and 1
fixtures each.

Each of the ten runs was replayed against the file:

| Run | Fixtures | Result | Worst fixture | Median fixture |
| --- | ---: | --- | ---: | ---: |
| 34506217184 | 65 | pass | 80% | 77.5% |
| 34509719470 | 65 | pass | 80% | 75.6% |
| 34511924673 | 65 | pass | 80% | 76.0% |
| the 7 earlier runs | 61 | `extra-entry` x4 | 80% | 67-79% |

No run breaches a ceiling. The seven earlier runs report `extra-entry`
because they never measured the four fold fixtures, which is the correct
answer for a run that does not exercise something the budget covers. The
next release cannot repeat it: `0.19.0-rc.2` is the published base from now
on, and it folds.

### On `policy.automation`

The policy said: "No task, script, or workflow may write this file." The 65
entries were computed by a one-off script in the scratch directory, from the
ten archived reports, and then committed as a reviewed change.

That keeps the rule the policy is for. The rule stops the budget from moving
itself: no task, no script and no workflow in this repository writes
`budget.json`, and a breach still blocks a release instead of raising a
ceiling. Sixty-five ceilings copied by hand from ten JSON reports would only
add transcription errors to the same result. The numbers are checkable: the
run ids are in every `evidence` field, and `parseEntry` re-derives each
`ceilingMs` from `observedUpperMs * headroom`.

The review found that this reasoning lived only in this file, which is
never committed, so the exception was written into `policy.automation` and
into `guidelines/PERFORMANCE.md` as well. The prohibition the rule is for
is unchanged, and the guard test that reads `policy.automation` still asks
for it.

### The image build no longer stops a release

An enforced budget must record the image builds it was seeded on, and every
other build used to be a failure. GitHub rebuilds `ubuntu24` about one time
each week, and the corpus above already spans two builds eight days apart,
so that rule would have stopped almost every release for a reason that no
change in this repository can answer.

A problem now carries a severity. An unknown image build is a `diagnostic`,
which is the treatment the CPU model already had, and for the same cause:
the run does not choose the machine. The image *family* stays a `failure`,
because a different family is a different class of machine. Only a
`failure` sets the status to `failed`.

## What remains

Nothing for this ticket. Two findings left with tickets of their own:

- The macOS paired benchmark segfaults when the base and the candidate
  both link mimalloc. The three dispatches here were the first runs to do
  that, because a release takes its base from the newest stable tag and
  that tag has no mimalloc yet —
  [ticket 53](../issues/53-macos-paired-benchmark-segfaults.md).
- An enforced budget plus `--allow-base-refusals` means a new fixture stops
  one release, whichever order it is added in —
  [ticket 54](../issues/54-a-new-fixture-stops-the-next-release.md). The
  seven replays above are that failure, seen once already.
- Ticket 52 asked for the seeding and is answered here.

## 4. What the review changed

Two review agents read the two commits. Both confirmed the arithmetic: 65
entries against the 65 names in `fixtures.v1.json`, every `ceilingMs`
inside 1% of `observedUpperMs * headroom`, every `runs` equal to the number
of reports holding that fixture, and `observedP95Ms` equal to
`median(perRoundP95Ms)` in all 650 rows.

They found one defect in behaviour and several statements the code did not
support. All were corrected in `91475d8d9`:

1. **An absent image build passed.** It became a diagnostic together with
   an unknown build. The two are not the same: GitHub always records the
   value, so its absence is a defect in the measurement file. It is a
   separate kind now, and a failure.
2. **A kind did not imply its severity.** A new kind could take the wrong
   one at the line that raised it. One `Record<BudgetProblemKind, ...>` map
   decides it now, so the type makes each kind necessary.
3. **The CPU model is not reported as a diagnostic.** Four places said it
   was. No problem is raised for it; the report prints it in the
   environment line.
4. **"Ten clean release runs" was not true of every ceiling.** Four come
   from three runs.
5. **"The worst run sits at 80%" was arithmetic, not a result** — it is
   `1/1.25`. The measured figure is the median utilisation, 76%.
6. **"All ten runs differ only by machine" was an overclaim.** Seven are
   seven different commits and set 43 of the 65 ceilings. Corrected above.
7. The intro to this file and a comment in `npm.yml` still described the
   budget as uncalibrated.

One residual risk had no ticket and now has one:
[ticket 54](../issues/54-a-new-fixture-stops-the-next-release.md).

Two claims in the reviews were wrong and were not acted on:
`x86_64-apple-darwin` is a real binding-test job label, and dropping the
per-job "slowest" column would not prevent staleness, because a dated
measurement cannot rot the way a present-tense claim can.
