# 55 — The unplugin concurrent-CSS test fails under load

**What to build:** `packages/unplugin/__tests__/bundledDevCss.test.ts`, the
test `renders concurrent requests independently without restoring stale
import dependencies`, fails sometimes in a full `pnpm test`, and passes
each time it runs alone.

Seen on 2026-09-10 while the budget work ran its gates. In one session:

- `pnpm test` passed twice, 97 of 97 tasks.
- `pnpm test` then failed once, only this test.
- The same file alone passed 3 of 3 times immediately after.
- `pnpm test` passed again, 97 of 97.

The test file is byte-identical to `9ac53b416`, and the change that was in
the tree touched only `crates/stylex-rs-compiler/benchmark`, three
workflows and `guidelines/PERFORMANCE.md`. So this is not new, and the
trigger looks like the parallel load of the full run.

The failing test writes to a temporary directory beside the package
(`packages/unplugin/.stylex-vite-*`) and the run prints many
`ENOENT ... global.css` lines. A temporary directory that another worker
removes, or a name that two workers share, would give this shape.

**Why it matters:** the failure is silent about its cause and it looks like
a real defect to the next person who sees it in CI.

**Blocked by:** None.

**Status:** done

- [x] The cause is known, and named here: the test replaced the served
      stylesheet with a plain `writeFile`, which empties the file before it
      fills it. The request that was still reading it then read nothing,
      which is where the empty and missing `global.css` bodies came from
- [x] The test passes under a full parallel run, repeatedly: 22 runs of the
      package suite under a synthetic load of 40 busy cores, no failure of
      this test
- [x] Each worker gets a temporary directory that no other worker can
      remove. It already did: every fixture root comes from `mkdtemp`, so
      no two workers can share or remove one. The temporary directory was
      not the cause

## Comments

**Already fixed before this ticket was read.** Commit `add9f214b`,
"test(unplugin): measure the behaviour, not the speed of the machine",
gave the test a `replaceFile` helper that writes a temporary file and
renames it over the target, so a reader gets the whole of one version or
the whole of the other. That commit is on the branch, and the checkboxes
above were confirmed against it rather than re-implemented.

**Two other tests in the package were still flaky**, found while proving
this one is not. Both are in `packages/unplugin/__tests__/vite.test.ts` and
both are fixed now:

- The `transformFixture` tests count how many times the stylesheet is
  transformed. A fixture writes its files and starts a dev server on the
  same directory a moment later, and the operating system reports those
  writes to the fresh watcher. The plugin then ran a hot update nobody
  asked for, and under load that update landed inside the test, giving a
  second and a third transform. Every dev-server fixture in the file now
  starts with the watcher off, so the only events are the ones a test
  emits itself.
- The refresh-retry tests read the retry chain after a fixed 200 ms pause.
  The chain is three attempts, each behind a 50 ms debounce, so under load
  the pause expired in the middle of it: the first window saw one attempt
  instead of three, and the rest were charged to the second window, which
  must be empty. The first window now waits until the attempt count stops
  moving, through a helper factored out of the stylesheet-read test. The
  second window stays a fixed pause, deliberately: it must find no attempt
  at all, and a wait for a count to stop moving cannot end on a count that
  never moves.

Measured over 25 runs of the package suite under the same synthetic load
that reproduced both: no failure, against about 1 run in 2 failing before.
