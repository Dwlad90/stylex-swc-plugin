# 01 — Align @property inherits with upstream

**What to build:** Dynamic styles that put a CSS variable behind a
pseudo-class emit the same `@property` registration as
`@stylexjs/babel-plugin` v0.19.0. Found during the media-query parity audit:
for a `:hover` variable in dynamic styles, this compiler emits
`@property --x-… { syntax: "*"; inherits: true; }` where upstream asserts
`inherits: false` — a genuine output divergence, and `inherits` changes
runtime cascade behavior, not just the hash.

Scope: locate where `@property` rules are generated for dynamic style
variables, compare against upstream's generation logic and its
`transform-stylex-create-test.js` dynamic 'media query with pseudo-classes'
expectations, fix to 1:1 parity, and pin with a transform-seam test.

**Upstream reference**: `~/Projects/Facebook/stylex` @ `5f51b2444` (the v0.19.0
release commit).

**Blocked by:** None — can start immediately.

**Status:** resolved

- [x] Reproduction confirmed first: the upstream dynamic pseudo-class case
      emits `inherits: false` upstream and `inherits: true` here (if it was
      fixed meanwhile, this ticket reduces to adding the test)
- [x] Upstream's rule for when `inherits` is true vs false is identified and
      ported 1:1 (verify whether it is unconditional or condition-dependent —
      do not assume from one example)
- [x] Transform-seam test pins the `@property` output for the dynamic
      pseudo-class case, matching upstream byte-for-byte
- [x] Full workspace `cargo test` green; snapshot churn only where the
      `@property` fix legitimately changes output

## Answer

`inherits` is condition-dependent, and the condition is **pseudo-element only**.
Upstream `stylex-create.js` computes
`const isPseudoElement = path.some((p) => p.startsWith('::'))` — a double
colon. This compiler matched on a single `':'`, so every pseudo-*class*
(`:hover`, `:active`, `:focus`, …) was also treated as a pseudo-element and
registered with `inherits: true`.

Fix: `crates/stylex-transform/src/transform/stylex/transform_stylex_create_call/mod.rs`
now tests `path.starts_with("::")`. Nested cases such as `::before` containing
`:hover` still resolve to `true`, because the `::before` segment is in the same
path — matching upstream's `.some(...)`.

Verified against upstream `5f51b2444` expectations byte-for-byte: the dynamic
`valid pseudo-class`, `pseudo-class generated order` and `media query with
pseudo-classes` cases all assert `inherits: false`, with the same variable
hashes this compiler emits. Every upstream `inherits: true` assertion (six of
them) has exactly one counterpart here — no more, no fewer.

Coverage: the byte-for-byte upstream guarantee comes from the pre-existing
`valid_pseudo_class`, `pseudo_class_generated_order` and
`media_query_with_pseudo_classes` snapshots. Added
`dynamic_style_in_hover_generates_at_property_with_inherits_false` as a named
regression pin for the bare pseudo-class case; its input has no upstream
counterpart, so it is a self-referential pin rather than an upstream-verified
one. `dynamic_style_in_after_generates_valid_at_property_with_inherits` and
`before_containing_pseudo_classes` pin the `inherits: true` side. Snapshot
churn is limited to four files, all `inherits: true` → `false` on
pseudo-class-only variables.

Also added a **Property registration** glossary term to
`crates/stylex-transform/CONTEXT.md`, since the pseudo-element/pseudo-class
distinction the bug turned on had no name in the crate's domain docs.

Not done (out of scope, worth a follow-up): the `"::"` prefix test is now the
eighth hand-rolled copy of the same rule across `stylex-css` and
`stylex-transform`, and this bug was one of those copies drifting to `':'`. A
shared `is_pseudo_element(&str)` helper would make the next drift impossible.
