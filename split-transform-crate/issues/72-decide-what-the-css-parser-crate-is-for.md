# 72 — What the CSS parser crate is for

**Settled.** `stylex-css-parser` is a library kept whole. The rest of the
workspace reaches one item —
`at_queries::media_query_transform::last_media_query_wins_transform` — and the
whole of the parser surface is to be used in time, so an export with no in-repo
caller is what a library looks like here and not dead code. The crate's own
`CONTEXT.md` already says as much under **unwitnessed parser**: a parser no
plugin run enters cannot be settled by comparing output.

This closes the question `CssValue` raised. It is reached by `lib.rs` and its
own tests alone, which is the same position as the rest of the surface, and it
stays.

**One module went, for a different reason.**
[64](./64-cover-the-transform-call-handlers.md) removed `flex_parser.rs`, which
was not part of that surface:

- The upstream Babel plugin has no parser of this shape at all. Its mirrored
  test corpus carries no `FlexParser`, no `FlexCombinators` and no
  `TokenParser`; upstream reads a value as text, through
  `transform-value-test.js` and `split-css-value-test.js`.
- The crate's own `CONTEXT.md` glossary names `TokenParser` as the way the
  crate is composed and never mentions this module, so it was outside the
  documented design.
- Its module doc described it as overcoming "the limitations of the current
  TokenParser system" -- a proposal rather than a port.
- `TokenListExt` was implemented with bodies whose own comments said they did
  nothing: "Current implementation uses stateless parsing", "Context is managed
  through parser composition rather than global state".
- It measured 0% behind three `coverage(off)` exclusions, and the test file
  registered from inside it asserted on `CssValue` without ever calling the
  module.

So the surface a library keeps is the one upstream has a counterpart for, or
one this compiler reaches. A local proposal with stub bodies is neither.

**Status:** resolved

- [x] The crate says which of the two it is: a library, kept whole.
- [x] An unused export is documented as a library surface --
      `CONTEXT.md` does that under **unwitnessed parser**.
- [x] The one module with no upstream counterpart and no caller is gone, and
      the crate keeps 100% without it.

## Comments

Raised by a review of the `flex_parser.rs` removal in
[64](./64-cover-the-transform-call-handlers.md), which asked whether
`CssValue` was next. It is not.
