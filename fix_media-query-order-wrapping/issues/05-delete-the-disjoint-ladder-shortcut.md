# 05 — Delete the disjoint-ladder shortcut

**What to build:** The behaviour reported in the issue. An author's ladder of
exclusive breakpoints now compiles to the same rule text, and therefore the same
class names, as the reference implementation produces — including the retained
contradictory branches that print as `not all` and the nesting around them.

This is a deletion, not an addition. The last-media-query-wins transform already
builds the negation chain correctly; what destroys it is a predicate in the
range merge that detects a disjoint breakpoint ladder and returns an empty
result, short-circuiting the distribution. The reference implementation has no
such predicate: a contradictory branch there recurses to the bottom and yields a
one-element result holding an empty disjunction, which the parent's filter keeps
because it drops only empty results, and which serialization prints as `not
all`. Our predicate returns a genuinely empty result, the same filter discards
it, and serialization unwraps the single survivor to the bare authored query.
Remove the predicate and its call site; change nothing else in canonicalization.

Reshaping the predicate to return the surviving shape is explicitly rejected:
its correctness claim has no reference-implementation line to check against, and
this is the change whose whole purpose is parity. Any expectation that ticket 02
flagged as disagreeing with the reference implementation is corrected here, with
the table row as its justification.

**Blocked by:** 04.

**Status:** done — see `../evidence/ladder-expansion.md`

- [x] The predicate and its call site are gone; nothing else in the
      canonicalization pass is modified.
- [x] All three of ticket 04's seams pass.
- [x] Every expectation changed in this ticket is named alongside the run that
      justifies it. **Answered rather than met as written.** The one
      expectation that changed is not in ticket 02's table and could not be:
      it parses a media query string directly rather than reaching one through
      a conditional value map, so the table's emitted-CSS measurement has no
      module to run. It is re-derived from its own reference run instead —
      `../evidence/ladder-expansion.md`. The single row ticket 02 *did* flag,
      `u03`, is untouched, because its third column shows the flag was an
      artefact of measuring a unit-seam expectation through post-sort CSS.
- [x] The Rust and JS suites pass, the JS suites against a fresh build.
- [x] The parity harness reports no unexpected rows for the reported input.
- [x] If reconciliation turns out not to fit one working session, the remainder
      is split into a follow-up ticket rather than left partly done.
