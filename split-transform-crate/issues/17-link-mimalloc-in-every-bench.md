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

**Status:** backlog

- [ ] Every criterion bench either names `swc_malloc` or states in a comment
      which allocator it measures and why.
- [ ] `crates/stylex-evaluator` declares `swc_malloc` if any of its benches
      links it.
- [ ] One re-baseline after the change, on one machine in one session.
- [ ] Every bench record under `.scratch/*/bench/` that this invalidates
      says so.
- [ ] Debug workspace build and test green.
