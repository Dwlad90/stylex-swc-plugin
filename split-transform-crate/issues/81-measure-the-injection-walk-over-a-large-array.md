# 81 — Measure the injection walk over a large array

**What to build:** a benchmark fixture that pairs a large top-level array of
styles with runtime injection, so the walk that places `_inject2(...)` calls is
measured on the shape that costs the most.

**Where the gap is.** `flush_pending_insertions` skips its whole walk when no
injection is queued, which is every module compiled with runtime injection off.
`benchmark/fixtures.v1.json` holds exactly one entry with
`runtimeInjection: true`, and the module it points at binds its `create` call to
a name -- the path the walk hashes once and does not descend. The two large
array fixtures, `lotsOfStyles.js` and `lotsOfStylesDynamic.js`, run with runtime
injection off.

So the one shape that pays for the walk -- a large array initializer, with
injection on -- is measured by nothing, and
[79](./79-inject-the-rules-an-object-in-an-array-declares.md) made that shape
more expensive on reasoning alone.

**What to settle:** whether the shape earns a fixture. `guidelines/PERFORMANCE.md`
asks a new fixture to earn its place, and this one would need a production and a
dev reading to be comparable with the rest. The alternative is a criterion bench
in `stylex-state`, which measures the walk without the whole compile around it
and needs no budget row.

**Blocked by:** None.

**Status:** resolved

- [x] The walk is measured over a large array initializer with runtime
      injection on.
      `crates/stylex-state/benches/injection_walk_bench.rs`, group
      `InjectionWalk`.
- [x] The figure is recorded, and a budget row is added if the fixture route is
      taken. The criterion route was taken, so there is no budget row. The
      figures are in the answer below and in
      `crates/stylex-state/docs/adr/0002-the-injection-walk-over-a-large-array-is-measured-and-kept.md`.

## Comments

Found by the performance review of
[79](./79-inject-the-rules-an-object-in-an-array-declares.md). The review put
the walk for `lotsOfStyles.js` at about 282,000 hashes rather than 21,700 once
the descent is on, which is the number that wants measuring rather than
reasoning about.

## Answer

The criterion route. No fixture, and no budget row. The record is
`crates/stylex-state/docs/adr/0002-the-injection-walk-over-a-large-array-is-measured-and-kept.md`;
this is the short form.

`crates/stylex-state/benches/injection_walk_bench.rs` measures the shape on the
corpus shape -- 3 namespaces per compiled object and 3 class-name declarations
per namespace, which is what `lotsOfStyles.js` compiles to. Criterion medians,
`aarch64-apple-darwin`, 2026-09-16, three runs within 3%, on a working desktop
at 69-72% idle.

| `create` calls | array, injection on | object, injection on | array, injection off |
| -------------- | ------------------- | -------------------- | -------------------- |
| 128            | 124.1 us            | 39.4 us              | 1.51 us              |
| 512            | 503.7 us            | 582.2 us             | 5.29 us              |
| 2048           | 2.025 ms            | 2.422 ms             | 20.6 us              |
| 21,733         | 22.44 ms            | --                   | 216.2 us             |

21,733 is the corpus size, so the headline figure is read off a clock rather
than extrapolated to it. The array shape is linear in the calls (4.06 and 4.02
per four-fold step, and 4% above linear over the last jump), it costs about
three times the object shape at the same style count, and injection off is about
1% of injection on.

**Why no fixture.** The walk is **22.44 ms**. Against the same module compiled
with injection on -- 1,333 ms on this machine, against 1,100 ms with it off --
that is **1.7%** of the compile that pays for it, and 9.6% of the 233 ms that
turning injection on costs. A fixture would not price the walk; it would price
the whole injection path, and a walk regression inside it sits well under the
1.25 headroom the ceilings carry. A new fixture also costs what
[54](./54-a-new-fixture-stops-the-next-release.md) records.

**Why no optimization.** Five options were considered and are written up in the
ADR, including one that is provably correct today -- narrowing the literal arms
to `Lit::Str`, worth about 8-10% of the walk and 0.15% of the compile. None
earns its keep against 1.7%.

**A gap this found beside the walk, now closed.** No module in the corpus was
priced with injection on over an array of `create` calls, so the 21% that
turning it on costs was invisible to the release gate, and the walk is only the
smallest tenth of that. `Feature - runtime injection at scale` is that reading:
`lotsOfStyles100.js`, 100 `create` calls in a top-level array, with
`runtimeInjection: true` -- 5.72 ms against 4.85 ms for the same file with
injection off, on this machine. It prices the whole injection path, which is
the right subject for a corpus row and the wrong one for this walk, so it does
not reopen the decision above.

Its ceiling is **seeded**, on 2026-09-18, by
`d8e6e5572 perf(stylexswc/rs-compiler): seed the runtime injection at scale ceiling`.
It landed with no ceiling first, which is the two-step route: the pull-request
leg does not read the budget, so the fixture lands green, and the release leg
then raises `missing-entry` for a fixture it measured with no committed
ceiling. Release run 35384266231 stopped on exactly that, for `0.19.0-rc.4`.

Three clean runs on the canonical environment --
`x86_64-unknown-linux-gnu`, Node 24.18.0, image `ubuntu24` build
`20260907.300.1`, ten rounds each -- gave the median-of-round p95 below.

| source                     | CPU       | p95        |
| -------------------------- | --------- | ---------- |
| run 35378604335            | EPYC 9V74 | 10.1048 ms |
| run 35384266231, attempt 1 | EPYC 9V74 | 10.0383 ms |
| run 35384266231, attempt 2 | EPYC 7763 | 13.7762 ms |

`observedUpperMs` 13.7762, `headroom` 1.25, `ceilingMs` 17.2203, `runs` 3.

Two of the three are the two attempts of one run. A re-run gets a fresh runner
and writes its own budget report, so it is an independent reading, but GitHub
keeps the run id of the dispatch it repeats. `evidence` therefore names the
attempt beside the id, and `guidelines/PERFORMANCE.md` says so under "Seeding a
new ceiling", or a reader follows the id to one artifact and finds one reading
where the entry claims two.

The seeding also corrected a figure the file carried. It said one CPU class is
about 15% slower than the other, from the original seeding. This fixture reads
36% between the same two classes, so the gap is per fixture and not a constant
a later seeding may lean on. The ceiling is still right, because it is taken
from the slowest class that appeared.

The review figure in the comment above -- about 282,000 hashes with the descent
on -- is not restated: nothing here counts hashes, and the clock reading above
replaces the need to.
