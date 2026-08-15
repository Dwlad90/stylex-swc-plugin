# 13 — Escapes are resolved inside strings

**What to build:** A developer writing `fontFamily: '"\2014 A", sans-serif'`
gets the class name the reference compiler generates, instead of a different one
produced from a value whose escape has been expanded.

Today this compiler resolves a CSS escape sequence inside a string to the
character it names — `"\2014 A"` becomes `"—A"`, `"\1F600"` becomes `"😀"` —
while `@stylexjs/babel-plugin` keeps the author's spelling. Different bytes
reach the hash, so the two compilers name different classes for the same source.
An escape *outside* a string (`My\ Font`) is already preserved by both.

This is the same family as the six divergences in the parent issue and was
surfaced by ticket 04's migrated coverage, not by a report. It is listed
separately because it is not one of the six and was not in the ticket 01
baseline: the corpus reached escapes only through `content`, which never gets to
value normalization at all, so the escape path had no oracle until `fontFamily`
entries were added.

Nothing new is expected to be needed for it: the lossless round-trip in ticket
07 preserves the source spelling by construction. The work is to confirm that
after the swap and flip the two `Reference::Diverges` cases in
`diverges_on_resolving_escapes_inside_a_string` to `Reference::Same`. If they do
not flip on their own, that is a gap in the port.

**Blocked by:** 07 — Swap normalization onto the ported pipeline. Confirmed
rather than assumed; see Investigation.

**Status:** ready-for-agent

## Investigation

Attempted ahead of 07 and abandoned, because there is nowhere to put the fix
that is not the pipeline replacement itself.

`stringify` emits with `minify: true`, and under that setting
`swc_css_codegen::emit_str` ignores the `raw` field entirely and writes
`minify_string(&n.value)` (`swc_css_codegen-24.0.0/src/lib.rs:1216-1227`). The
escape is already resolved in `value` by the time the parser hands it over, so
the source spelling is gone before codegen is reached.

Writing `raw` back into `value` before codegen does not work either:
`minify_string` escapes `\` to `\\` (`lib.rs:2759-2761`), so `\2014 A` would
come out as `\\2014 A` — a second, worse divergence.

Repairing after codegen is impossible in principle, not merely awkward: once
the escape is resolved, `"—A"` produced from `"\2014 A"` is indistinguishable
from `"—A"` the author wrote. Reconstructing the difference would mean
re-reading the original input and splicing — the "reverse a previous step"
pattern the parent spec names as the thing this effort exists to end.

So the fix is to stop routing strings through that codegen, which is exactly
what 07 does. Nothing to carry forward but this note.

- [ ] An escape inside a string survives normalization with its source spelling
      intact, for both a BMP and an astral-plane codepoint
- [ ] The parity harness reports `identical` for the three `fontFamily` escape
      entries in `edge.json`
- [ ] The migrated cases in
      `crates/stylex-css/src/css/tests/value_normalization_parity_test.rs` carry
      `Reference::Same` and assert the source spelling
- [ ] An escape outside a string is still preserved, as it is today
