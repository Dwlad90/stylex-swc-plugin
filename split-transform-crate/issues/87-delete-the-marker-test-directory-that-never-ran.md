# 87 — Delete the marker test directory that never ran

**What to fix:** `crates/stylex-transform/tests/transform_default_marker_test/`
was never registered. `tests/transform.rs` declares 37 `mod` lines and this
directory is not among them, so neither of its two cases has ever executed.

The snapshot tree is the proof: `default_marker_named_import` and
`default_marker_namespace_import` have snapshots only under
`__swc_snapshots__/tests/transform_stylex_when_test/`, never under this
directory's own path.

This is the repository's known silent failure mode, and closing it in
`tests/legacy/` was this branch's headline win (`e59514c1b`). The same sweep
missed this directory.

**How it was answered.** Deleted. The file is a near-duplicate of the live
`transform_stylex_when_test/default_marker_transform.rs` -- the only difference
is that the live cases write `export const classNames` where these write
`const classNames` -- so the behaviour is already covered, and registering the
directory would have added a second snapshot of the same two cases.

**Blocked by:** None.

**Status:** resolved

- [x] The directory is gone
- [x] No orphan snapshot is left under its path
- [x] The live cases in `transform_stylex_when_test` still pass
