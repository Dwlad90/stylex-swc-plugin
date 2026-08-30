# 02 — Record the pre-split baseline

**What to build:** There is no defect to reproduce here, so the pre-flight for
this refactor is a measurement. Without it, "the split improved maintainability"
is unfalsifiable and no later ticket can prove it did not regress performance or
coverage.

Capture, at a named commit, the numbers every later ticket is measured against,
and store them beside this spec so any agent picking up a later ticket can diff
against them without re-deriving anything.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] Full suite recorded green, run directly — never piped into a pager or tail, or the exit code is the pager's.
- [ ] All seven criterion benches recorded with machine and profile noted.
- [ ] Coverage output saved, including the current exclusion list verbatim.
- [ ] Cold build time recorded.
- [ ] Incremental check time recorded after touching the state manager.
- [ ] Source-line counts recorded per crate, so the end-state table has a starting point.
- [ ] The commit the measurements describe is recorded explicitly.
- [ ] Documentation only — no source changes in this ticket.
