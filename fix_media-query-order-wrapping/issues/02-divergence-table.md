# 02 — Re-derive every media-query expectation from the reference

**What to build:** A recorded table, one row per existing media-query
expectation in this repository, giving that expectation beside the output the
reference implementation actually produces for the same input, and flagging
every disagreement. Nothing in production code or in any test changes. This is
the evidence promised before any snapshot is rewritten: two earlier efforts in
this area pinned expectations against this same reference version and one of
them added the shortcut ticket 05 deletes, so no expectation here is trusted
without being re-derived.

Cover the transform's own unit expectations, the canonicalization suite, and the
computed-bounds suite. Record the reference implementation's resolved version in
the table, because the lockfile rather than an exact range holds it.

**Blocked by:** None — can start immediately.

**Status:** done — see `../evidence/divergence-table.md`

- [x] Every media-query expectation in the repository appears as a row.
- [x] Each row carries the input, this repository's expectation, and the
      reference implementation's actual output for that input.
- [x] Disagreements are flagged and counted; a zero count is a valid result and
      is stated as such.
- [x] The reference implementation's resolved version and the file it was
      resolved from are recorded in the table.
- [x] No production code, test, or snapshot is modified — evidence: a clean
      working tree apart from the table itself.
