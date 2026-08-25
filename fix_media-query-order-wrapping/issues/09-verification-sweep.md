# 09 — Verification sweep: at-rule order and the ordering gate

**What to build:** Confidence, recorded, that this change moved nothing it did
not intend to move. Three things are checked against the reference
implementation and changed only where it disagrees.

At-rule sorting compares the final key text, so a rewritten key that is much
longer sorts differently among its siblings than the authored one did. The
reference implementation sorts the same rewritten strings, so the comparator is
expected to need no change — but the emitted order is checked, not assumed.
Second, whether the conversion of a parse failure into the
invalid-media-query-syntax error fires on exactly the inputs the reference
implementation's outer catch fires on. Third, whether the media-query ordering
option's default matches, including that opting out hashes the authored spelling
instead.

A recorded "no change needed", with the comparison that establishes it, is a
valid and expected outcome for all three.

**Blocked by:** 05.

**Status:** done — see `../evidence/sweep.md`

- [x] Emitted at-rule order for a conditional value map mixing rewritten media
      keys with other at-rules is compared against the reference implementation
      and the result recorded.
- [x] The inputs on which the invalid-syntax refusal fires are compared against
      the reference implementation's, and any gap named. Twenty-two inputs
      compared; the unbalanced-parenthesis family closed, and three grammar
      gaps named and filed as ticket 13.
- [x] The ordering option's default is confirmed against the reference
      implementation, and the opt-out path is confirmed to hash the authored
      spelling.
- [x] Any change is justified by a recorded disagreement; where there is none,
      the comparison itself is the deliverable.
