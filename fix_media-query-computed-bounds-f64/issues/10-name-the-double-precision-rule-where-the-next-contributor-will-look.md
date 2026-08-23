# 10 — Name the double-precision rule where the next contributor will look

**What to build:** a contributor adding a new numeric CSS type finds the rule
stated in the crate's domain glossary rather than having to infer it from the
surrounding code — numeric CSS types hold a double, and print through the shared
JavaScript-number helper, because the printed spelling reaches the class-name
hash and is therefore observable.

Without this, the next numeric type added to the crate reintroduces the whole
class of divergence that tickets 01 through 09 closed.

**Blocked by:** 07, 08, 09.

**Status:** done

- [x] The crate's `CONTEXT.md` language section states the double-precision and
      shared-formatter rule, in the glossary's existing voice
- [x] The rule names why the spelling is observable, not only what to do
- [x] `pnpm format:check` passes, including the markdown line width

## Closing note

Delivered. `CONTEXT.md` gains a **Double-precision number** entry, placed after
**CSS type** so a contributor reading about the types meets the rule
immediately.

It states both halves as one rule, and the reason they are one: the printed
spelling reaches the class-name hash, so it is observable output rather than a
debugging detail. Each half is grounded in what it actually costs -- an `f32`
field rounds `28.81 - 0.01` to `28.8` where the official compiler emits
`28.799999999999997`, and Rust's own formatting spells `1e21` with twenty-two
digits, names an overflow `inf`, and keeps the sign on a negative zero. It
links to `stylex-utils`' own glossary for the formatter.
