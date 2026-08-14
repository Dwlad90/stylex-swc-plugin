# 04 — Align normalize AST shapes with upstream

**What to build:** `MediaQuery` normalization produces the same AST shapes as
upstream v0.19.0, so ported structural assertions hold and future upstream
changes port mechanically. Serialized strings are already equivalent; this
ticket removes the shape divergences and their compensations:

- no early return for an empty flattened `and` list (upstream lets the merge
  run);
- a contradiction yields the bare `not all` media keyword, not an
  `and`-wrapped one;
- the `not`-arm special case that unwraps the wrapped shape (needed only
  because of the previous point) is dropped.

**Blocked by:** 02 — Collapse contradictory negation branches (same
merge/normalize interplay). Can run in parallel with 03.

**Status:** resolved

- [x] Normalize's `and`, `or`, and `not` arms are 1:1 with upstream
      `MediaQuery.normalize`
- [x] Emitted strings unchanged for every existing test input (contradictions
      still print `not all`, `not (not all)` still collapses to `all`)
- [x] Any structural assertion adjusted in existing Rust tests is justified by
      the upstream test file, never by making the test pass
- [x] Full workspace `cargo test` green; no snapshot churn

## Answer

Three edits to `MediaQuery::normalize` in
`crates/stylex-css-parser/src/at_queries/media_query.rs`:

1. Dropped the `flattened.is_empty()` early return. Upstream lets the merge
   run; `merge_intervals_for_and(vec![])` already returns an empty vec, so the
   `merged.is_empty()` branch produces the same `not all` for that input.
2. The `merged.is_empty()` branch now yields the bare
   `MediaKeyword("all", not = true)` instead of wrapping it in an `And`.
3. Removed the `not`-arm `And(len == 1)` special case, which existed only to
   unwrap the shape point 2 no longer produces.

Upstream justification for the shapes: `parse-media-query-test.js:2038`,
`@media not ((min-width: 500px) and (max-width: 600px) and (max-width: 400px))`,
snapshots a bare `media-keyword` at the top level — the inner contradiction
must therefore normalize to a bare `not all` keyword for the `not` arm's
first branch to flip it, with no `And` unwrapping in between.

Four Rust tests encoded the old wrapped shape and were retargeted, not
loosened:

- `parse_media_query_test.rs` — `media_px_1000_to_700_range` and
  `media_max_width_200px_and_not_max_width_300px` now expect the bare
  keyword. Both still assert `@media not all` as the emitted string.
- `media_query_coverage_test.rs` —
  `normalize_and_with_empty_result_after_merge_returns_and_with_not_all`
  renamed to `…_returns_not_all_keyword` and expects the bare keyword.
- `media_query_coverage_test.rs` —
  `normalize_not_of_and_single_not_all_returns_all` covered a branch that no
  longer exists; replaced by `normalize_not_of_contradictory_and_returns_all`,
  which drives the same `Not(contradiction) → all` outcome through the path
  that actually runs.

No consumer outside these tests pattern-matches the `And`-wrapped keyword.
Full workspace `cargo test` green (4,548 tests); `pnpm format:check` green;
`git status` shows no snapshot churn.
