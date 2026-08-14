# 03 — Remove the disjoint shortcut and dead code

**What to build:** The last-media-query-wins transform follows upstream
`dfsProcessQueries` exactly: it always builds the negation chain and relies on
`MediaQuery` normalization to collapse redundant clauses. The
disjoint-ranges shortcut — a compensation for the merge bug fixed in ticket
02, with no upstream counterpart — is deleted, along with its helpers, the
silent keep-authored fallback on re-parse failure, and the vestigial no-op
`last_media_query_wins_transform_internal`.

User-visible behavior must not change: the shortcut's outputs already matched
upstream where it engaged, and after ticket 02 the general path produces the
same strings.

**Blocked by:** 01 — Pin issue #1250 end-to-end; 02 — Collapse contradictory
negation branches.

**Status:** resolved

- [x] The disjoint shortcut and its helper functions
      (`are_media_queries_disjoint`, `extract_width_height_range`,
      `ranges_overlap`, `normalize_media_query_syntax`) are removed; the
      negation path runs unconditionally
- [x] The silent keep-authored-on-parse-failure fallback is gone; an
      unparseable query still surfaces as `INVALID_MEDIA_QUERY_SYNTAX`
      (parity with upstream's throw)
- [x] `last_media_query_wins_transform_internal` is removed after confirming
      no callers outside its own tests
- [x] Full workspace `cargo test` green with zero snapshot churn (unit,
      transform, fixture, and app snapshots all byte-identical)
- [x] `merge_intervals_for_and`'s vestigial `Result<_, String>` is dropped for
      a plain `Vec` — no arm in its body constructs `Err`, which also makes
      `merge_and_simplify_ranges`' `Err(_) => rules` fallback unreachable.
      Upstream has no error channel here, and `String` errors contradict the
      repo's `anyhow` policy. Surfaced by both reviewers on ticket 02, which
      left the signature alone to keep that fix confined to the merge's return.

## Comments

Pre-verified while resolving ticket 02: with the shortcut bypassed
(`if false && are_media_queries_disjoint(...)`) the full css-parser suite is
green at 2185 tests, `mixed_min_max_width_with_many_disjoint_ranges` included.
The deletion should therefore be mechanical — if a test goes red, suspect the
deletion, not the merge.

Resolved as predicted: mechanical, no test needed changing for behaviour. The
css-parser suite lands at 2179 — the six removed tests all targeted deleted
code paths (three no-op `_internal` tests, two `are_media_queries_disjoint`
unit tests, two `normalize_media_query_syntax` unit tests), and the disjoint
module's end-to-end invalid-key panic test was kept, moved into
`transform_media_coverage`.

Two extras beyond the checklist, both fallout of the deletions rather than new
behaviour:

- `merge_and_simplify_ranges` disappeared with its error channel. Once the
  `Err(_) => rules` fallback and the redundant empty-check were gone it was a
  pure alias for `merge_intervals_for_and`, so `normalize` now calls the merge
  directly. This also drops the `rules.clone()` the fallback needed.
- `parsed_media_pairs` lost its `String` element. The media key was only ever
  read by the disjoint check; with that gone the tuple is
  `(KeyValueProp, MediaQuery)`, and the panic message reads the key from the
  loop binding before it is discarded.

Verification: full workspace `cargo test` green (22 binaries, 0 failures),
`cargo clippy --all-targets` warning-free, `rs-compiler` rebuilt and the full
JS suite green (63 turbo tasks) with `git status --porcelain` showing only the
four modified `.rs` files — zero snapshot churn.
