# 44 — Close the declarator list to the crates above it

**What to build:** `declaration_index` in `stylex-state` holds a **position**
into `declarations`, so the two must grow together. `push_declaration` is the
only writer that keeps them in step, and the field beside it was `pub`. A crate
above could therefore push straight past the writer, and the binding it added
would be invisible to `declaration_of`.

`push_declaration`'s own documentation admitted the hole, and a comment beside
the guard called the field `pub(crate)` while it was in fact `pub`. The guard
that catches such a push is a `debug_assert`, which release builds do not carry,
so the hole was open in exactly the profile that ships.

The change landed in `f78ab2dbf` without a ticket. This file records it, and
records the one decision inside it that a reader could otherwise read as an
oversight.

**Blocked by:** None.

**Status:** resolved

- [x] `declarations` is `pub(crate)`, and a crate above reads it through the
      `declarations()` accessor
- [x] The one caller outside the crate takes the accessor
- [x] The comment beside the `debug_assert` and the field agree
- [x] A test covers the invariant the accessor protects: the index and the list
      answer the same question after a push
- [x] The workspace gate is green in **debug** -- never `--release`

## Comments

**Why `BindingWrites` keeps four public fields.** The same commit left that
type's fields public, which reads as an inconsistency and is not one.
`BindingWrites` is a **parameter object**: it is built by name in the transform
and handed to `adopt_binding_writes` in one move. Its fields carry no invariant
between them, and nothing derives a position from one field into another. The
reason `declarations` had to close is the index beside it; there is no such
index here, so there is nothing for an accessor to protect.

**What this does not fix.** A push from **inside** `stylex-state` can still
bypass the writer. That is deliberate: the crate holds the invariant, the
`debug_assert` catches a mistake in a debug run, and closing the field to its
own crate would leave the writer unable to write. The change moves the boundary
to where the type system can hold it, which is the crate edge.
