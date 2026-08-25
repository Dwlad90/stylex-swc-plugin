# 10 — Glossary and decision record

**What to build:** The vocabulary and the reasoning, written where the next
person will find them. The crate glossary already names media query
canonicalization and the last-media-query-wins transform, and already says that
a contradiction prints as `not all`; it gains the two facts this work
established — that contradictory disjunction branches are retained rather than
pruned, and that a collision between rewritten keys drops a declaration.

A decision record covers what a reader will otherwise try to undo. The wrapped
output is rejected by lightningcss's minifier, which refuses the doubly
parenthesised form, and we emit it knowingly: a rejected stylesheet fails
loudly, a class-name divergence fails silently, and matching the reference
implementation is the point of this compiler. The dropped declaration is
recorded as a ported upstream defect rather than as intended design, so that it
is revisited when the upstream report is resolved, and the recursion bound's
number is recorded with its provenance so that it is not deleted as arbitrary.

> **One glossary entry already landed, in ticket 01.** `Range merge boundary`
> names the boundary and the inner-recovery-against-outer-refusal distinction,
> because ticket 01 introduced that vocabulary and leaving it unnamed cost the
> next reader. This ticket's two facts are deliberately *not* in it -- that
> contradictory branches are retained, and that a key collision drops a
> declaration -- because neither is true until tickets 05 and 07 land. Extend
> the existing `Media query canonicalization` and
> `Last-media-query-wins transform` entries with those, rather than restating
> the boundary.

**Blocked by:** 05, 07, 08.

**Status:** done, with one criterion partly open — the upstream report numbers
do not exist until ticket 11 files them

- [x] The glossary states that contradictory branches are retained, and that a
      key collision drops a declaration.
- [x] A decision record states the lightningcss rejection, that it is knowing,
      and why it loses to the hash-divergence argument.
- [x] The decision record marks the declaration loss as a ported defect and
      names the upstream report that tracks it. **Partly open.** It is marked
      as a ported defect and points at ticket 11, which is what tracks the
      filing; the report numbers themselves cannot be named until that ticket
      has approval to file. The paragraph says so and says where they go.
- [x] The recursion bound's number and provenance are reachable from the
      decision record.
- [x] Prose wraps at eighty columns and the glossary's existing avoid-term
      convention is followed.
