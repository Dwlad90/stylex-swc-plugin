# 100 — Read a CSS type value the way it was matched

**What to fix:** `get_css_value` in `crates/stylex-structures/src/base_css_type.rs`
matches a property on the pair a shorthand name stands for, and then takes the
value from the property as the object holds it. The two readings disagree for
one spelling.

`{ syntax: '<color>', value }` matches, because `prop_named` reads the shorthand
`value` as the pair `value: value`. The caller then asks the raw property for
its pair, a shorthand holds none, and the build stops with "Expected key-value
property" rather than reading the value the author gave.

**How old it is.** Pre-existing. The reader spelled the walk out twice before
[90](./90-stop-the-copies-that-go-straight-to-waste.md) made it one function,
and both spellings had this shape.

**Why it is not fixed here.** No source reaches it. Both callers --
`stylex_define_vars.rs:70` and `stylex_create_theme.rs:105` -- are given a
declaration the evaluator rebuilt, and a rebuilt object holds no shorthand.

Carrying the matched pair out to the caller means answering a `Cow` and reading
it back through two arms, one of which only a shorthand takes. That arm is
reachable from no source, so it would be an uncovered region on a crate the
workspace gate measures -- the same trade recorded for N3 in
[99](./99-decisions-this-remediation-recorded.md). A note at `prop_named` says
the two readings disagree, which is what the next reader needs.

**What would settle it:** a producer that hands `get_css_value` an object it did
not rebuild. Then the shorthand is reachable, the arm is coverable, and the fix
is worth making.

**Blocked by:** None.

**Status:** ready-for-human

- [ ] A shorthand `value` beside a `syntax` reads the value the author gave
- [ ] The case is reachable from a source before the arm is added
