# 54 — A new benchmark fixture stops the next release

**What to build:** Adding a benchmark fixture for a new feature now stops
the next release, and the message does not say why.

`npm.yml` passes `--allow-base-refusals`, so a fixture that the published
base cannot compile leaves the paired run. The budget is `enforced` now, and
it raises `extra-entry` for a committed ceiling that the run did not
measure. `extra-entry` is a failure. So:

1. Someone adds a fixture for a feature that is not published yet, and adds
   its ceiling.
2. The next release runs against the last published base, which refuses the
   fixture, and the run drops it.
3. The budget sees a ceiling with no measurement and stops the release.

The reverse also holds: add the fixture and no ceiling, and the run raises
`missing-entry` as soon as a base can compile it. Either order stops a
release once.

This is the same shape as the problem
[ticket 52](./52-seed-the-benchmark-budget.md) had to solve by hand, and
the ten replays in [`../bench/ticket-20.md`](../bench/ticket-20.md) show it:
the seven runs from before the fold shipped each report four
`extra-entry` failures against the committed file.

**Worth thinking about.** The budget cares about the candidate only. A
fixture that the base refuses still has a candidate measurement, and the
run throws it away because a _paired_ comparison needs both sides. If the
budget read the candidate before the pairing removes the fixture, this
whole class of failure would not exist, and ticket 52 would not have needed
three dispatches.

**Blocked by:** None.

**Status:** done

- [x] Adding a fixture that the published base cannot compile does not stop
      a release
- [x] The `extra-entry` message says that the base refused the fixture,
      when that is the cause
- [x] A test covers a fixture that only the candidate can measure

## What was done

The first and third points landed with
`f105006b8 fix(stylexswc/rs-compiler): keep a base refusal out of the comparison only`.
A fixture a subject refuses keeps the subjects that answered for it, the
budget reads the candidate over that set, and `release-leg.test.ts` runs the
real producer into the real budget and verdict.

The second point is the `extra-entry` message. One kind carried one sentence
for two answers, so a reader of a failed release could not tell a stale
ceiling from a fixture the subject could not compile. `checkCoverage` now
reads the whole fixture list beside the measured subset and says which:

- `budget entry "X" names no benchmark in this run` -- the entry is stale.
- `budget entry "X" ran, but the base "npm@0.18.6" has no measurement for it`
  -- the fixture is correct and that subject could not compile it.

The message does not say "refused". The raw stats record which subjects
measured a fixture, not why the others did not: a refusal, a zero rule count
and a broken subject process all reach the file the same way. The message
names what the file shows and stops there.

The `missing-entry` half the ticket body names is unchanged. A fixture with
no committed ceiling must still stop a release, because a ceiling needs three
clean seeding runs and no message can supply them.
