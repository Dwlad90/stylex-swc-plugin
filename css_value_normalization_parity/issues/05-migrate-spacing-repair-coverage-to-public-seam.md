# 05 — Migrate spacing-repair coverage to the public seam

**What to build:** The same treatment as ticket 04, for the other body of
implementation-coupled tests: those asserting against the hand-rolled whitespace
repair pass and the helpers around it.

This coverage is the more valuable of the two. It is where years of individually
reported defects accumulated — a function result immediately followed by a unit,
URL bodies containing characters that look like CSS syntax, comments inside
values, adjacent quoted strings, non-ASCII content, percentage followed by a
number. Each of those assertions exists because someone hit the bug. The
implementation they guard is deleted in ticket 07; the knowledge must not be.

Re-express every case against the public value normalization entry point,
asserting current behaviour, annotated with the harness verdict from ticket 01.
Green before the swap and after it.

**Blocked by:** 01 — Babel differential parity harness.

**Status:** ready-for-agent

- [ ] Every input covered by the existing spacing-repair tests is covered by a
      case at the public normalization entry point
- [ ] Expectations assert current behaviour, so the suite is green before any
      pipeline change
- [ ] Each case records whether it matches the reference compiler or is a known
      divergence scheduled to change, sourced from the harness
- [ ] The regressions preserved explicitly include: a function result directly
      followed by a unit with no space inserted, URL bodies copied verbatim,
      comments passed through intact, adjacent strings kept separate, non-ASCII
      content preserved, and the leading-zero treatment of negative decimals
- [ ] The value-extraction and rule-structure helpers used only by these tests
      are covered through the public entry point too, so their removal in ticket
      09 loses nothing
- [ ] No new assertion references the repair pass or any helper ticket 07
      removes
- [ ] The original test files are left in place for now — deletion is ticket 09
