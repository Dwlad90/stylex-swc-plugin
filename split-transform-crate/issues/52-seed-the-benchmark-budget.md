# 52 — Seed the benchmark budget, once a run can measure every fixture

**What to build:** [Ticket 20](./20-reprice-the-addon-ci-and-budget.md) priced
the release matrix and computed 61 ceilings from seven clean LTO runs, but
left `crates/stylex-rs-compiler/benchmark/budget.json` at
`pending-calibration`. The ceilings and the full corpus are in
[`../bench/ticket-20.md`](../bench/ticket-20.md). Two things stop the seed,
and both are about the gate rather than about the numbers.

**1. Four fixtures are never measured.** The registry holds 65; every release
run measures 61. `npm.yml` passes `--allow-base-refusals`, and the published
base refuses `Feature - engine fold`, `Feature - calls that do not fold` and
their dev variants — it cannot fold `trim` or `Math.sign`. An enforced budget
raises `missing-entry` for a measured fixture with no ceiling, so the first
release that takes a folding base fails on four of them. `0.19.0-rc.2` is
published and does fold both, so that release is the next one.

**2. The image pin expires faster than releases happen.** An enforced budget
must pin at least one runner `ImageVersion`, and any other build is a
recalibration failure. The seeding corpus already spans two builds eight days
apart, because GitHub rebuilds `ubuntu24` at about that rate. A pin written
at seeding time is stale before the next release, and the release then fails
on the machine rather than on the code.

The second one is a decision about the policy, not a measurement. The file
says image drift is "an explicit recalibration failure, not a comparison",
which is the right instinct and the wrong period: it makes the gate block
every release. Either the release can re-seed itself — which the policy bans,
and rightly — or the rule needs a tolerance the policy states in so many
words. Settle that before seeding, so the first enforced release is not the
one that discovers it.

**Blocked by:** ticket 20 for the corpus. The four fixtures need a release
whose base folds; nothing in this repo can bring that date forward.

**Status:** resolved

- [x] A release run measures all 65 fixtures on the canonical runner. Three
      of them: 34506217184, 34509719470 and 34511924673, each dispatched
      with `previous-version=0.19.0-rc.2`, which folds `trim` and
      `Math.sign`.
- [x] The runner-image rule says what happens on a rebuild, and the reason is
      written next to it. An unknown build is a diagnostic, in
      `lib/budget.ts`, in the `policy.environment` text and in
      `guidelines/PERFORMANCE.md`.
- [x] `budget.json` is `enforced`, with one ceiling per measured fixture, the
      seeding runs named in `evidence`, and the headroom justified.
- [x] A release passes the budget gate without a hand edit. All three runs
      that measure 65 fixtures were replayed against the file and pass, with
      the worst fixture at 80% of its ceiling.

## Answer

Both blockers are gone, by different routes.

The four fold fixtures needed a published base that can fold. `0.19.0-rc.2`
is that base, and it was already on npm, so three dispatches were enough.
The corpus is ten clean runs on the canonical environment: seven earlier
ones measuring 61 fixtures, and three measuring all 65 on `6c30e7d88`.

The image pin needed a decision rather than a measurement, and the decision
was to make an unknown build a diagnostic. The file already treated the CPU
model that way for the same cause, so this is one rule where there were two.

Full record: [`../bench/ticket-20.md`](../bench/ticket-20.md).
