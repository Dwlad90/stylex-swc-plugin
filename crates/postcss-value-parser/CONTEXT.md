# postcss-value-parser

Reads a CSS declaration value into a loose token list and spells it back out,
losing nothing on the way. Third-party code, kept in a crate of its own so that
the boundary is visible: nothing in here is StyleX logic, and it depends on
nothing.

The vocabulary below is the library's own, not this project's. Where a term
here collides with one elsewhere in the workspace, the other context wins and
this one is qualified — see the entries' _Avoid_ lines.

## Language

**Value scanner**:
This crate. It answers what a value _says_, character for character, and never
what it _means_. Not the
[typed value parser](../stylex-css-parser/CONTEXT.md), which is parser
combinators over CSS grammar and is a different upstream package with a
different purpose. Not the swc CSS parser either.
_Avoid_: tokenizer, lexer, value parser, CSS parser

**Node**:
One token. A single record with a kind discriminant and optional fields, not an
enum — deliberately, because the callers reading it are written as "inspect the
kind, then assign to the value field", and an enum would force each of them
into a match with a catch-all arm.
_Avoid_: token, AST node, item

**Node kind**:
Which of the seven token shapes a node is — word, string, div, space, comment,
function, unicode-range. A word is deliberately coarse: an identifier, a
number, a dimension, a hex colour, an importance annotation and an operator
inside `calc()` are all words.
_Avoid_: node type, token type

**Div**:
A separator — `,`, `:` or `/` — carrying the whitespace on either side of it as
`before` and `after` rather than letting that whitespace become its own nodes.
The name is the library's; it has nothing to do with HTML.
_Avoid_: separator node, delimiter, punctuation

**Before / after**:
The whitespace captured on a node rather than emitted beside it. Populated for
divs and for functions only; a string or a comment never carries either.
_Avoid_: padding, leading/trailing space, gap

**Unclosed**:
A flag saying the token ran off the end of the input — a string with no closing
quote, a comment with no `*/`, a function with no `)`. It is recorded, never
raised: the scan has no failure mode.
_Avoid_: error, invalid, malformed

**Source offset**:
Where a node starts and ends, counted in **bytes**. Load-bearing rather than
bookkeeping: a caller decides whether a token sits inside a function by
comparing offsets rather than by tracking state. Three degenerate inputs push
an end offset one byte past the input, which is behaviour and is pinned as
such.
_Avoid_: span, position, range, index

**Override**:
A callback consulted for every node before it is spelled out, free to replace
that node's text outright or to decline. Reaches nested nodes, so a function
inside a function can be replaced without the outer one knowing.
_Avoid_: custom stringifier, visitor, formatter hook

**Dimension**:
A word split into the number it starts with and whatever follows. The unit half
is whatever the number scan did not consume, whether or not it is a real CSS
unit: `10zz` splits as cleanly as `10px`. A word that does not start with a
number has no split at all, which is a different answer from a split with an
empty number.
_Avoid_: unit, measurement, quantity

**Walk**:
Visiting every node, descending into functions. Outside-in by default;
inside-out when bubbling. It lends out a node, never the list holding it, so a
structural edit to a node list happens outside the walk.
_Avoid_: traverse, visit, iterate

## Deliberate divergences from the JavaScript

Three, each chosen rather than inherited. Everything else is parity, and a
difference found outside this list is a bug.

**Byte offsets, not UTF-16 indices**. A node's start and end count bytes, where
the JavaScript counts UTF-16 code units. Rust strings are indexed by byte, so
carrying the JavaScript's numbers would make every offset unusable without a
conversion at each use. The parity generator rebases the two against each other.

**Override order among siblings**. A parent is consulted before its children in
both, but siblings go left to right here and right to left in the JavaScript,
which builds its result by prepending from the tail. The text produced is
identical, so only a stateful override can tell. Reproducing the JavaScript's
order would mean giving up the single output sink to recreate an accident of
string concatenation. Pinned by `an_override_is_consulted_left_to_right`.

**Deep nesting aborts rather than throwing**. The scan is iterative and survives
any depth, but spelling a tree back out, walking it, and dropping it are all
recursive — so a value nested tens of thousands deep exhausts the stack, which
in Rust is an abort no caller can catch, where the JavaScript throws a
`RangeError`. An embedder must bound depth before parsing; `stylex-css` rejects
past 64 in `scan_value_structure`.
