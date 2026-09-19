# 38 — Let the release gate read a base it cannot fully measure

**What to build:** The paired benchmark leg of `.github/workflows/npm.yml`
measures the candidate against the last published release. That base is behind
the candidate by every feature landed since, so a fixture pricing one of those
features has no second side: the base refuses it, and the leg failed on a
refusal that says nothing about the candidate.

Two parts. The runner learns to drop a fixture the base cannot measure, with a
line saying which fixture and why, while a fixture the *candidate* refuses
still fails the leg. And the verdict names the breach that reproduced rather
than every flag it was given, because a banner listing flags reads as several
regressions when there is one.

This was filed after the fact. The work landed as `8689f614e` and `6e502c6ca`
while ticket 37 was being closed, and had no ticket of its own.

**Blocked by:** None.

**Status:** resolved

- [x] `bench:revisions` takes `--allow-base-refusals`, drops a fixture only the
      base refuses, and reports which
- [x] A fixture the candidate refuses still fails the leg
- [x] Both paired legs of `npm.yml`, native and Alpine, pass the flag, each
      with a comment saying why
- [x] The verdict banner names the breach that reproduced
- [x] `guidelines/PERFORMANCE.md` documents the flag and when it applies

## Comments

**Filed retroactively.** The change is right and is covered by
`benchmark/__tests__/runner.test.ts` and `benchmark/__tests__/verdict.test.ts`.
What was missing was the ticket, which is why ticket 37's first criterion went
stale — see the note there.
