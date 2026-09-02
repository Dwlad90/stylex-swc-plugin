# 26 — Clear the documentation, layering and bookkeeping drift

**What to build:** The crate graph this work produced is genuinely acyclic and
strictly downward, but the documented layer ladder does not describe it: five
crates are documented far above their real depth, one of them seven rungs
high, so a reader concludes that CSS generation may call the evaluator. The
ladder should be generated from the manifests rather than drawn by hand, which
was the stated intent of the commit that deleted the previous hand-drawn
version.

Alongside that, this branch took several deviations from its own spec that are
each harmless and none of which is recorded. Two commits landed performance
optimisations the spec excludes from scope. A test file lost five helper
definitions to a new scaffolding module, where the spec permits editing only
import lines. Thirty snapshot headers were regenerated for staleness that
predates this work. Two functions were split during move commits, where the
spec forbids splitting — one of them specifically so the coverage tool could
instrument the halves separately, which is worth settling as a question in its
own right, since it happened in the same branch that exempted two crates from
that tool. A relocated macro stamps the caller's file and line into panic
messages, so every moved call site now reports a different location on
stderr — unavoidable, but an observable output change that no test asserts.

The spec's boundary criterion claims nothing with a counterpart in the
reference implementation was cut. Two readers were in fact split across
several crates. Neither has a counterpart, so no translated unit was severed
and no behaviour diverged — but line-for-line comparison against the reference
evaluator now spans three crates, and the next parity investigation should be
told that before it starts looking.

Finally a handful of naming and comment drift, and one decision to record:
this branch made the only real test removal in 408 files, deleting a dead
path-resolution helper along with its eight tests and a workspace dependency.
The code was genuinely dead and the deletion is defensible; what is missing is
someone saying so on purpose.

**Blocked by:** 21

**Status:** ready-for-agent

- [ ] The layer ladder is regenerated from the manifests, so no crate is
      documented above its real depth
- [ ] Amendments record the two performance commits, the test-helper move, and
      the regenerated snapshot headers
- [ ] The two functions split during move commits are recorded; all were
      verified behaviour-neutral, so the record is that they happened
- [ ] Whether a coverage tool may dictate a function boundary is settled one
      way or the other
- [ ] The boundary-criterion rebuttal is recorded where a parity investigation
      will find it: comparison against the reference evaluator now spans three
      crates
- [ ] The panic location shift is noted in the pull request description
- [ ] A decision record states that the state crate must not be split further
      until its evaluation callback stops taking the state manager by mutable
      reference — that alias, not the module count, is the knot
- [ ] The stale transform crate description, the doc example that teaches an
      `.expect` call against the guidelines, and the odd dependency ordering
      are fixed
- [ ] The two conflicting same-named type helpers are resolved, and the type
      comparison one of them powers stops string-comparing Rust type names in
      favour of a real type
- [ ] The upstream-measurement comment deleted by the diagnostics move is
      restored. This is *not* the same as the branch's removal of porting
      wording, which the spec explicitly requires — that scrub stays
- [ ] The dead path-resolution helper's deletion is settled: either recorded
      as deliberate dead-code removal, or its eight tests and both production
      copies are restored, with the dropped workspace dependency matching the
      decision
- [ ] The workspace gate is green in **debug** — never `--release`, since the
      fixture suite only guards debug: `format:check`, `lint:check`,
      `lint:shell`, `typecheck` and the test suite, each run directly rather
      than piped into a pager, whose exit code would mask a failure. Re-run
      `typecheck` after committing, because the pre-commit hook rewrites code
