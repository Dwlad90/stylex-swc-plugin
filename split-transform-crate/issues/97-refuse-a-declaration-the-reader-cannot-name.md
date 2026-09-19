# 97 — Refuse a declaration the reader cannot name

**What to fix:** `read_declarations` (`stylex-state/src/folded_value.rs`) let a
property it could not read fall out of an `if let` with no arm. A
`PropOrSpread::Spread`, a `Prop::Method`, a `Prop::Getter`/`Setter` or an
unexpanded `Prop::Shorthand` was dropped, and the module was emitted as though
the author never wrote the declaration.

The list arm in the same file refuses exactly these with
`stylex_unimplemented!`, and `get_key_values_from_object` refuses them too. This
was the odd reader out, and a wrong output is worse than a stopped build.

**How it was answered.** A `let ... else` that refuses, so the reader now reads
like its siblings.

Latent, not live: `object_expression.rs:178-192` expands a shorthand and stops
on a method or a spread before this reader sees one. Both tests that pinned the
old silent skip -- one in `folded_value_test.rs`, one in
`parse_nullable_style_tests.rs` -- now pin the refusal instead, and the first
grew from one case to six, one per spelling the reader cannot name plus one
that proves the refusal comes before the names beside it are written.

**Blocked by:** None.

**Status:** resolved

- [x] Every spelling the reader cannot name is refused
- [x] One case per spelling
- [x] The refusal stops the whole object rather than shortening it
