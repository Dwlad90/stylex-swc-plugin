# 04 — Extract the state-index crate

**What to build:** The **state manager** composes two lookup structures that let
it answer "which declarator, which call, which span" with one hash probe instead
of a scan of the module. They are pure Rust-only machinery with no counterpart,
and one of them has no internal dependencies at all — the true leaf of the whole
crate.

Give them their own crate. The state manager keeps its fields and composes the
types from across the boundary; its own struct and method surface are untouched,
because it corresponds to a single unit on the comparison side.

This is the tracer bullet for the entire split: it proves the extraction
pipeline end to end — create the crate, move code and its tests, satisfy the
coverage gate, place it in the documented DAG — at the smallest possible stake.

Note that the coverage gate runs across the whole workspace, so this crate is
gated the moment it exists. Note also that the coverage tool keeps only the
best-covered instantiation of a generic, so a generic index can read as fully
covered while one instantiation is untested.

**Blocked by:** 02 — Record the pre-split baseline.

**Status:** ready-for-agent

- [ ] Both index structures live in the new crate.
- [ ] The state manager remains one struct with an unchanged method surface.
- [ ] The `Rc`-sharing is preserved — cloning the state manager for a dynamic style's callback must not become a deep copy.
- [ ] The unit tests covering the indices move with them.
- [ ] The crate reaches zero uncovered lines and zero uncovered regions, with every generic instantiation exercised.
- [ ] The crate has a `CONTEXT.md` defining its vocabulary and a row in the context map.
- [ ] The crate is placed in the documented layer list.
- [ ] Manifest matches the conventions of existing crates; no publish key.
- [ ] No re-export facade is added to the transform — call sites use the new crate path.
- [ ] Benches diffed against the baseline; no regression outside noise.
- [ ] Lockfile regenerated and committed with this change.
