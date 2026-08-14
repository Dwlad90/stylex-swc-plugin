# 02 — Collapse contradictory negation branches like upstream

**What to build:** A user who authors (or whose last-query-wins negations
produce) a media query whose DeMorgan-distributed branches are all
contradictions gets the collapsed canonical range upstream emits — never a
mangled `or` expression with leftover `not` clauses.

The defect: the interval merge's DeMorgan branch returns its distributed `or`
only when non-empty; upstream `mergeIntervalsForAnd` returns it
unconditionally (empty branches filtered, multi-rule branches wrapped in
`and`, an empty `or` kept as-is) and lets serialization collapse empty and
single-child `or`s. Our serializer already matches upstream, so the fix is
confined to the merge's return.

This is user-reachable today without the transform: a directly authored query
with two contradictory `not(range)` clauses normalizes wrong on parse.

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] The DeMorgan branch of the interval merge returns the distributed `or`
      unconditionally, 1:1 with upstream v0.19.0 `mergeIntervalsForAnd`
- [x] Unit test: parsing
      `@media (min-width: 900px) and (max-width: 1440px) and (not ((min-width: 600px) and (max-width: 800px))) and (not ((min-width: 400px) and (max-width: 500px)))`
      canonicalizes to `@media (min-width: 900px) and (max-width: 1440px)`
- [x] Regression harness from the audit goes green: with the disjoint shortcut
      bypassed, `mixed_min_max_width_with_many_disjoint_ranges` produces
      upstream's expected strings (the shortcut itself is removed in ticket
      03, not here)
- [x] All existing css-parser tests stay green

## Comments

Red/green captured: before the fix the new parser-seam test reproduced the
audit's exact mangled string —

```
@media ((min-width: 900px) and (max-width: 1440px) and (not ((min-width: 400px) and (max-width: 500px))) and (not (min-width: 600px))) or ((min-width: 900px) and (max-width: 1440px))
```

Both expectations were verified against the upstream v0.19.0 parser directly
(`MediaQuery.parser.parseToEnd(...)` on `style-value-parser/lib`), not derived
from the Rust output.

A second parser-seam test pins the branch the fix newly reaches: when *both*
DeMorgan arms contradict, the `or` comes back empty and serialization collapses
it — `@media (min-width: 900px) and (max-width: 1000px) and (not ((min-width: 500px) and (max-width: 1500px)))`
→ `@media not all` (upstream-verified). Before the fix this input fell through
to the un-distributed rule list.

The recursive calls now propagate with `?` instead of silently dropping an
`Err` branch. No path in `merge_intervals_for_and` returns `Err`, so this is
behaviour-preserving, and it matches upstream, which has no error channel.

`demorgan_both_branches_empty_yields_no_or_rules` in the coverage suite named
the removed `if !or_rules.is_empty()` check and asserted nothing. Its input
never reached the DeMorgan branch at all — negating a contradictory range is a
tautology that `normalize` collapses to `all` first — so it was renamed to
`not_of_a_contradictory_range_normalizes_to_all` and given a real assertion.

Ticket 03's harness pre-verified: with the disjoint shortcut bypassed the full
css-parser suite is green (2185 tests), so removing it should be a clean
deletion.

Gate at resolve time: workspace `cargo test` green (all 25 targets),
`cargo fmt --all --check` clean, `cargo clippy -p stylex_css_parser
--all-targets` clean. The JS/visual sweep belongs to ticket 06.
