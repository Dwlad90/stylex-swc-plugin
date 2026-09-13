# 17 — Link mimalloc in every bench, or say why a bench does not

**What to build:** Seven of the repo's nine criterion benches measure the system
allocator where the shipped `.node` runs mimalloc. Make the allocator a decision
each bench states, rather than one it inherits by accident.

`crates/stylex-transform/Cargo.toml` declares `swc_malloc` as a dev-dependency
and its comment reads as if that covers the crate's benches. It does not. Rust
links a dev-dependency only where a target names it, so the one line that makes
the allocator real is `use swc_malloc as _;`, and exactly one bench in the tree
writes it: `crates/stylex-transform/benches/transform_consumers_bench.rs:77`.
The comment was corrected in ticket 09 to say so; this ticket is the fix behind
the correction.

**Why it matters most for the evaluator.** The three benches ticket 09 moved --
`engine_fold_bench`, `evaluate_bench`, `evaluate_depth_bench` -- are the
allocation-heavy ones. The memo key hashes a whole subtree, the engine fold
prints its expression to source, and `EvaluateResultValue` is cloned along the
way. Allocator behaviour is precisely the axis those three sit on, and it is
the axis they do not currently measure faithfully. The evaluator crate has no
`swc_malloc` dependency at all, so this is a manifest change and not only a
`use` line.

**This closes every existing series.** Every number in `.scratch/*/bench/` for
these ids was measured against the system allocator. Changing the allocator
changes them all, so the change needs one clean re-baseline and a note in each
affected record saying which side of the change its numbers are on. That cost
is the reason this is one deliberate ticket rather than a line added quietly to
the next bench that gets touched.

**Decide per bench, do not blanket-apply.** A bench that measures parsing or
path resolution allocates little and gains nothing but a slower link. The
deliverable is that every bench either links the allocator or carries a comment
saying which allocator it measures and why that is the right one.

**Found by:** the performance review on ticket 09.

**Status:** resolved

- [x] Every criterion bench either names `swc_malloc` or states in a comment
      which allocator it measures and why. All fourteen name it; see "One
      decision for all fourteen" below for why none takes the comment.
- [x] `crates/stylex-evaluator` declares `swc_malloc` if any of its benches
      links it. Five crates gained the dev-dependency.
- [x] One re-baseline after the change, on one machine in one session. Done
      at `a4b520b45`, criterion baseline name `mimalloc`: 14 bench targets,
      268 measurements, all green. The record is
      [`../bench/ticket-17.md`](../bench/ticket-17.md).
- [x] Every bench record under `.scratch/*/bench/` that this invalidates
      says so. Six records and `baseline.md` carry the same note, and each now
      names the re-baseline to read instead. `bench/ticket-20.md` is the
      seventh record and takes no note: it holds CI wall-clock and fixture
      budget numbers from the published addon, which has always linked
      mimalloc, so no criterion series runs through it.
- [x] Debug workspace build and test green.

## Comments

**One decision for all fourteen benches.** The ticket says to decide per bench
and warns that a bench which measures parsing or path resolution "gains nothing
but a slower link". Every bench takes the line anyway. The reason is that the
alternative costs more than it saves:

- The comment is not free either. `ALLOCATOR: system` has to say why the system
  allocator is the right one for that bench, and that claim needs a measurement
  to stand on. A bench that allocates little today can allocate more after any
  change to the code under it, and nothing tells the reader when the claim went
  stale.
- A bench that links the allocator measures what production measures. That is
  true whether or not the bench is allocation-heavy, so the uniform choice is
  never wrong, while the split choice is wrong as soon as a bench moves.
- The cost is small and it is paid at build time. Five crates take
  `swc_malloc` as a **dev-dependency**, and cargo builds a dev-dependency for
  `cargo test` as well as for `cargo bench`, so the debug test build compiles
  mimalloc even though no test binary links it. The published addon and the
  workspace `build` are untouched, because neither reads a dev-dependency.

`ALLOCATOR: system` stays in the reader for the case this decision does not
cover: a bench whose subject *is* the system allocator. None exists today.

**Why the re-baseline is a separate run.** The measurement invalidates six bench
records and the pre-split baseline, so it has to be one session on one machine
with nothing else running. `guidelines/PERFORMANCE.md` states the rule, and
every affected record now carries a note saying which side of the change its
numbers are on, and names the re-baseline to read instead. No series across
`aa2925f06` may be compared, and the note is what stops a later reader from
doing so by accident.

**The run happened at `a4b520b45`.** Fourteen bench targets, 268 measurements,
one session, criterion baseline name `mimalloc`. The record is
[`../bench/ticket-17.md`](../bench/ticket-17.md), and it carries the machine,
the command, the medians and what the numbers may not be read to say.

Two things came out of it that are not this ticket's work:

- The machine was held at 0% idle for 19 hours by twenty orphaned load
  generators left by an earlier session. `tools/rebaseline.sh` now refuses to
  measure a saturated machine, because the condition was invisible until
  somebody looked.
- Two benches measure nothing.
  [Ticket 57](./57-two-benches-measure-nothing.md) records both.
