# postcss-value-parser

Reads a CSS declaration value into a loose token list and spells it back out.
Third-party code, kept in a crate of its own so the boundary is visible: nothing
here is StyleX logic, and it depends on nothing. The vocabulary below is the
library's own.

## Language

**Value scanner**:
This crate. It answers what a value _says_, character for character, and never
what it _means_. Not the
[typed value parser](../stylex-css-parser/CONTEXT.md), which is parser
combinators over CSS grammar.
_Avoid_: tokenizer, lexer, value parser, CSS parser

**Node**:
One token: a record with a kind discriminant and optional fields, not an enum.
The callers read it as "inspect the kind, then assign to the value field", and
an enum would force a catch-all arm into each of them. A field left `None` is
the JavaScript's `undefined`, which is a different answer from an empty string.
_Avoid_: token, AST node, item

**Node kind**:
Which of the seven token shapes a node is — `word`, `string`, `div`, `space`,
`comment`, `function`, `unicode-range`. A word is deliberately coarse: an
identifier, a number, a dimension, a hex colour, `!important` and an operator
inside `calc()` are all words.
_Avoid_: node type, token type

**Div**:
A separator — `,`, `:` or `/` — carrying the whitespace on either side of it as
`before` and `after`, rather than letting that whitespace become its own nodes.
_Avoid_: separator node, delimiter, punctuation

**Before / after**:
The whitespace captured on a node rather than emitted beside it. Populated for
divs and functions only, and the two mean different things: on a div it is the
whitespace around the separator, and on a function the whitespace just inside
the parentheses.
_Avoid_: padding, leading/trailing space, gap

**Unclosed**:
A flag saying the token ran off the end of the input — a string with no closing
quote, a comment with no `*/`, a function with no `)`. It is recorded, never
raised: the scan has no failure mode and no input produces an error.
_Avoid_: error, invalid, malformed

**Source offset**:
Where a node starts and ends, counted in **bytes**. Load-bearing rather than
bookkeeping: a caller decides whether a token sits inside a function by
comparing offsets. Two degenerate inputs push an end offset one byte past the
input — an unclosed string, and a trailing backslash — which is pinned as
behaviour.
_Avoid_: span, position, range, index

**Override**:
A callback consulted for every node before it is spelled out, free to replace
that node's text or to decline. It reaches nested nodes, so a function inside a
function can be replaced without the outer one knowing.
_Avoid_: custom stringifier, visitor, formatter hook

**Dimension**:
A word split into the number it starts with and whatever follows. The unit half
is whatever the number scan did not consume, real CSS unit or not: `10zz`
splits as cleanly as `10px`. A word not starting with a number has no split at
all, which is a different answer from a split with an empty number.
_Avoid_: unit, measurement, quantity

**Walk**:
Visiting every node, descending into functions. Outside-in by default;
inside-out when bubbling, where the callback's return value is ignored and
descent is unconditional. It lends out a node, never the list holding it, so a
structural edit happens outside the walk.
_Avoid_: traverse, visit, iterate

## Deliberate divergences from the JavaScript

Each was chosen rather than inherited. Everything else is parity, and a
difference outside this list is a bug.

**Byte offsets, not UTF-16 indices**. A node's start and end count bytes, where
the JavaScript counts UTF-16 code units. Rust strings are indexed by byte, so
carrying the JavaScript's numbers would need a conversion at each use.

**Override order among siblings**. A parent is consulted before its children in
both, but siblings go left to right here and right to left in the JavaScript.
The text produced is identical, so only a stateful override can tell. Pinned by
`an_override_is_consulted_left_to_right`.

**Deep nesting aborts rather than throwing**. The scan is iterative, but
spelling a tree back out, walking it and dropping it are all recursive — so a
deeply nested value exhausts the stack, which in Rust is an abort no caller can
catch, where the JavaScript throws a `RangeError`. An embedder must bound depth
before parsing; `stylex-css` rejects past 64.

**The `/*/` round trip**. The comment scan starts at the opening `/`, so it
finds its terminator inside `/*/` and `/*/ x */` spells back out as `/**/ x */`.
This is the one input the scanner does not reproduce, and it changes class
names.

Two further differences are recorded in the crate's own documentation: a dead
second slash test left out of the word scanner, and a walk callback with no
sibling list.
