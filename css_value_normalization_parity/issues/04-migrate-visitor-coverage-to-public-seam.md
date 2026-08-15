# 04 — Migrate normalizing-visitor coverage to the public seam

**What to build:** The tests covering the CSS-AST normalizing visitor currently
assert against that visitor directly — they build a stylesheet, run the visitor
over it, and check the serialized result. That code is deleted in ticket 07, and
if these tests go with it, years of accumulated regression knowledge goes too.

Re-express that coverage against the public value normalization entry point
instead: same inputs, same expectations, asserted one level up at a boundary
that survives the rewrite.

Do this **now, while the old implementation is still live**. That is the point
of the ticket. These tests pass before the swap and after it, which turns ticket
07 from an unguarded rewrite into a change with a net under it. Migrating this
coverage after the swap would mean landing the riskiest change in the effort
with nothing watching.

Each migrated case carries a verdict from the ticket 01 harness: either this
expectation matches what the reference compiler produces, or it is one of the
known divergences and will change at ticket 07. Recording that verdict up front
is what makes the ticket 07 diff reviewable — every changed expectation was
predicted, and an unpredicted change is a bug.

**Blocked by:** 01 — Babel differential parity harness.

**Status:** ready-for-agent

- [ ] Every input covered by the existing visitor tests is covered by a case at
      the public normalization entry point
- [ ] Expectations assert current behaviour, so the suite is green before any
      pipeline change
- [ ] Each case records whether it matches the reference compiler or is a known
      divergence scheduled to change, sourced from the harness rather than from
      judgement
- [ ] The behaviours preserved include: millisecond-to-second conversion and its
      below-threshold exception, zero-dimension unit handling across angles,
      timings, fractions and percentages, zero handling inside functions, the
      custom-property exemption from zero normalization, camel-case value
      conversion for the properties that get it, and font-size conversion under
      both settings of its option
- [ ] No new assertion references the visitor, the stylesheet type, or any other
      internal that ticket 07 removes
- [ ] The original visitor test files are left in place for now — deletion is
      ticket 09, once the swap has proven the migrated coverage holds
