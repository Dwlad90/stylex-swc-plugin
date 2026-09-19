# 56 — Measure each subject in its own process on the darwin benchmark legs

**What to build:** The paired release gate loads the base and the candidate
into one process. On darwin that pair works only while at most one of the
two bindings links mimalloc, which is a fact about the *previous* release
and not about the change under measurement. The gate must stop depending
on it.

## Why it is open

[Ticket 53](./53-macos-paired-benchmark-segfaults.md) made the failure
visible: the guard now sees a binding an installer hoisted, reads the
allocator of both bindings, and stops the load with a sentence instead of
a bare SIGSEGV. What it did not do is let the measurement happen.

`release.yml` takes the base from the newest **stable** tag, so today the
base is `0.18.6`, which links no mimalloc, and the two darwin legs measure
both subjects and pass. The first release whose base is `0.19.0` or later
carries mimalloc on both sides. That release stops on both darwin legs with
the guard's message, and `publish` needs `benchmark`.

## What makes it more than a small change

`runRounds` is fixture-outer: for each fixture it builds a balanced
schedule and runs every round. One round times both subjects inside one
`tinybench` instance, and the round is the unit the verdict engine
bootstraps. Two things must survive whatever replaces the single process:

- **Round-level alternation.** Every subject occupies every timing position
  once per block. Running all of the base rounds and then all of the
  candidate rounds turns thermal drift into an apparent regression, which
  is the reading the balanced schedule exists to prevent.
- **The timed loop.** A subject that answers over IPC cannot be timed. The
  process boundary has to sit outside the loop, around a whole round, not
  around a transform.

Those two together point at one child process per (subject, round), with
the parent holding the schedule and merging the answers into the same
`revisions-raw-stats.v1.json` the verdict engine reads now. Ten rounds and
two subjects is twenty children for a leg, so the cost is process start-up
and not measurement.

## Notes for whoever fixes this

- The pair that cannot share a process is known before either subject
  loads: `isDualLoadUnsafe`, `findNativeBindings` and `linksMimalloc` in
  `benchmark/lib/native-bindings.ts` answer it from the files. So the
  split can be taken only where it is needed, and Linux keeps the
  one-process path that gates every release today.
- Nothing about the schema changes. A merged file must be the file a
  one-process run writes, or `bench:verdict` and `bench:budget` read a
  shape they were not given.
- The alternative the ticket title does not take -- publishing a release
  whose base links mimalloc and watching the darwin legs -- is not a fix.
  It is the same gamble the gate takes now.

**Blocked by:** None.

**Status:** done

- [x] A darwin leg measures a base and a candidate that both link mimalloc
- [x] Round-level alternation survives the split, and a test shows it
- [x] The merged raw stats are the same shape as a one-process run
- [x] Linux keeps measuring both subjects in one process

## Answer

The split is per fixture and per round, not per run. `runRounds` keeps the
schedule, the fixture selection and the statistics; two of its steps -- the
rule count and the timing of one round -- became strategies the caller
supplies. The single process is still the default, so every platform but
one runs the code it ran before.

`bench-worker.ts` is the child. It is given the fixture rather than told
where to find it, for two reasons. The fixture it measures is then the
object the parent selected, so the two cannot drift. And it needs no
module that reads the manifest -- which matters, because `lib/types.ts`
reads a value off `dist/index.js`, and importing that loads *this*
package's binding. A child that read the manifest would hold the candidate
binding before it loaded the subject it was asked about, and would end the
way the single process does.

The two sides pass files, not pipes: the release workflow sets
`DEBUG: napi:*`, so anything parsed out of stdout would be parsed out of a
stream napi writes to as well.

### What was measured

Darwin arm64, base `0.19.0-rc.2` and the local build, both linking
mimalloc -- the pair that gave SIGSEGV:

| Run | Before | After |
| --- | --- | --- |
| One fixture, two rounds | exit 139, no result | exit 0, both subjects timed, ratio 1.00 |
| 61 fixtures, two rounds, base `0.18.6` | n/a | exit 0, 4 fixtures dropped with the base's own sentence |

A ratio of 1.00 is the answer to check: `0.19.0-rc.2` and the candidate are
the same code.

**The split does not move the number.** Measured on the pair that can share
a process, so both paths could run it:

| Path | base `0.18.6` / candidate, per round |
| --- | --- |
| One process | 3.699, 3.752 |
| A process each | 3.719, 3.755, 3.716, 3.725 |

### What it costs

61 fixtures, one round: 64.9 s in one process, 86.3 s in a process each.
That is 21.5 s a round, or 0.18 s a child, which is process start-up and
nothing else. The release leg runs ten rounds, so it pays about 3.6 min on
a leg whose slowest target takes 27.6 min. The job allows 50.

### Why there is a flag

`--separate-processes` asks for the split where the platform does not need
it. Without it the new path would run on macOS alone, and only against a
base that links mimalloc, which no leg has yet -- so it would ship
untested. A test names the flag, which puts the path under CI on every
platform.
