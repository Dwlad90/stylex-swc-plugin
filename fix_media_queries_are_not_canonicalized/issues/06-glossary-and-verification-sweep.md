# 06 — Glossary term and final verification sweep

**What to build:** The feature is finished as a whole: the CSS parser crate's
domain glossary names "media query canonicalization" so future contributors
find the concept, and the full repository gate proves the branch is
release-ready end to end.

**Blocked by:** 03 — Remove the disjoint shortcut; 04 — Align normalize AST
shapes; 05 — Port the remaining upstream test gaps.

**Status:** resolved

- [x] "Media query canonicalization" entry added to the CSS parser crate's
      CONTEXT.md, cross-linked from the existing last-media-query-wins entry
- [x] Full workspace `cargo test` green
- [x] `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check` green
- [x] `rs-compiler` rebuilt before running `pnpm test` so the JS suites
      exercise the new native binary; suites green
- [x] App visual/unit snapshots show zero churn
- [x] Commits follow the conventional sequence agreed in the spec, on this
      branch

## Answer

Landed as `ccb3e418e` —
`docs(stylex-css-parser): name media query canonicalization in the glossary`.

The glossary entry deliberately names all three phases that canonicalize a
query, because the first draft said the rewrite happens "on construction" and
code review caught that as false: the `min-`/`max-` conversion and the 0.01
epsilon are in the range parsers (`media_query.rs:1197-1213`), only the
flatten/merge/DeMorgan work is in `MediaQuery::normalize`, and the
contradiction's `not all` comes out of serialization
(`media_query.rs:408-410`). Two further claims were narrowed against the
source: DeMorgan distribution fires only for a `not` over a two-clause `and`
(`media_query.rs:546-551`), and the canonical key reaches the class hash via
the sorted at-rule modifier string
(`convert_style_to_class_name.rs:44-47`), which the transform only produces
for nested `@media` keys while `enableMediaQueryOrder` is on — its default
(`flatten_raw_style_object.rs:65`, `stylex_options.rs:66`).

One fix beyond the ticket text, in the same file: the crate header claimed the
parser handles "never a stylesheet, a selector, or a rule", which contradicted
the `at_queries` module living in the crate and would have sent a reader
looking for media queries to `stylex-css`. It now admits at-rule preludes.

Verification gate, all green:

- `cargo test --workspace` — 26 suites, 0 failures
- `pnpm typecheck`, `pnpm format:check`, `pnpm lint:check`
- `rs-compiler` rebuilt, then `pnpm test` — 63/63 tasks
- `pnpm test:visual` — 72/72 tasks; zero snapshot churn, working tree clean
  apart from the doc edit

`next-rspack-example` `/rsc` on Chrome Mobile failed the first visual run with
76 scattered single-pixel diffs (0.01% of the image, no layout shift) and
passed on re-run and on a full clean re-run — render flake, not style churn.
Worth knowing it can flake if CI reports it.
