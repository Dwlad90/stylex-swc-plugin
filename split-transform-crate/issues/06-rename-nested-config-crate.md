# 06 — Rename the nested-config crate

**What to build:** A crate currently holds the name `stylex-evaluator` while
containing no evaluation at all — its entire surface is flattening the nested
config objects that `defineVars` and its siblings accept. It was created whole
for the nested-API work; nothing was ever moved into it. The mismatch is severe
enough that the context map carries an explicit note steering readers away from
it and back to the transform for the general evaluator.

Rename it to describe what it does, freeing the name for the code that earns it
in the following tickets, and delete the note the mismatch made necessary.

This is a pure rename: no logic changes, no files move between crates.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

- [ ] The crate is renamed in its manifest and on disk.
- [ ] Every dependent manifest points at the new location.
- [ ] The context-map row is renamed and the redirect note is deleted.
- [ ] The crate's own `CONTEXT.md` heading matches the new name.
- [ ] Its position in the documented layer list is updated.
- [ ] No behaviour changes; the suite is green with no test edits beyond import lines.
- [ ] The old name is free for use.
- [ ] Lockfile regenerated and committed.
